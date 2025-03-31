use serde::Serialize;
use specta::Type;
use ts_rs::TS;

use crate::{
    application::{events::GameEvent, turns::TurnManager},
    core::types::Direction,
    domain::{
        entity::{self, Entity, EntityId},
        world::World,
        world_position::WorldPosition,
    },
};

use super::world_changes::{ChangeType, WorldChange};

pub enum ProcessState {
    ProcessingTurns,
    WaitingForPlayer,
}

pub struct GameState {
    pub world: World,
    pub turn_manager: TurnManager,
    pub process_state: ProcessState,
}

#[derive(Serialize, TS, Type)]
#[ts(export)]
pub struct PlayerActionResult {
    pub changes: Vec<WorldChange>,
    pub world: World,
    pub remembered_entities: Vec<Entity>,
}

impl GameState {
    pub fn new(world: World, turn_manager: TurnManager) -> Self {
        Self {
            world,
            turn_manager,
            process_state: ProcessState::ProcessingTurns,
        }
    }

    pub fn handle_player_action(
        &mut self,
        action: GameEvent,
    ) -> Result<PlayerActionResult, String> {
        let player = self.world.get_entity_by_id(&self.world.player_id);
        if self
            .turn_manager
            .current_entity()
            .is_some_and(|e| e == self.world.player_id)
            && player.is_some()
        {
            let changes_from_own_action = self.handle_event(action);
            let changes_from_others = self.progress_turns();
            let changes: Vec<WorldChange> = changes_from_own_action
                .into_iter()
                .chain(changes_from_others.into_iter())
                .collect();
            let visible_world = self.get_visible_world();

            visible_world
                .entities
                .values()
                .into_iter()
                .for_each(|entity| {
                    if let Some(seen_entity) = self.world.get_entity_by_id_mut(&entity.id) {
                        if let Some(seen_at) = seen_entity.pos() {
                            seen_entity.last_seen_at = Some(seen_at);
                        }
                    }
                });

            let seen_entities: Vec<EntityId> = visible_world
                .entities
                .values()
                .into_iter()
                .map(|e| e.id)
                .collect();
            let remembered_entities = self
                .world
                .entities
                .values()
                .into_iter()
                .filter(|entity| {
                    entity.last_seen_at.is_some() && !seen_entities.contains(&entity.id)
                })
                .cloned()
                .collect();

            Ok(PlayerActionResult {
                changes,
                world: visible_world,
                remembered_entities,
            })
        } else {
            Err("Not player's turn".to_string())
        }
    }

    pub fn get_visible_world(&self) -> World {
        let mut visible_world = self.world.clone();
        let player_pos = visible_world
            .get_entity_by_id(&self.world.player_id)
            .unwrap()
            .pos();
        visible_world.entities.retain(|_, e| {
            if let Some(entity_pos) = e.pos() {
                self.world
                    .has_line_of_sight(&player_pos.unwrap(), &entity_pos)
            } else {
                false
            }
        });
        visible_world
    }

    fn handle_event(&mut self, event: GameEvent) -> Vec<WorldChange> {
        match event {
            GameEvent::MoveByDirection(entity_id, dir) => {
                let entity = self.world.get_entity(entity_id);
                let old_pos = entity.and_then(|e| e.pos());

                let new_pos = match (old_pos, dir) {
                    (Some(pos), Direction::North) => Some(WorldPosition::new(pos.x, pos.y - 1)),
                    (Some(pos), Direction::South) => Some(WorldPosition::new(pos.x, pos.y + 1)),
                    (Some(pos), Direction::East) => Some(WorldPosition::new(pos.x + 1, pos.y)),
                    (Some(pos), Direction::West) => Some(WorldPosition::new(pos.x - 1, pos.y)),
                    (None, _) => None,
                };

                if let Some(new_pos) = new_pos {
                    let entities_in_pos = self
                        .world
                        .get_entities_by_pos(&new_pos)
                        .to_vec()
                        .into_iter()
                        .cloned()
                        .collect();
                    if let Some(entity) = self.world.get_entity_mut(entity_id) {
                        let can_move = entity.can_move_to(entities_in_pos);

                        if can_move {
                            entity.set_pos(Some(new_pos));
                            return vec![WorldChange::new(
                                entity_id,
                                ChangeType::Move(old_pos.unwrap_or(new_pos), new_pos),
                            )];
                        }
                    }
                }
                vec![]
            }
            GameEvent::SkipTurn => vec![],
        }
    }

    fn progress_turns(&mut self) -> Vec<WorldChange> {
        let mut changes: Vec<WorldChange> = Vec::new();
        while let Some(next_entity) = self.turn_manager.next_turn() {
            if let Some(_entity) = self.world.get_entity_mut(next_entity) {
                let is_player_turn = next_entity == self.world.player_id;
                if is_player_turn {
                    self.process_state = ProcessState::WaitingForPlayer;
                    break;
                } else {
                    self.process_state = ProcessState::ProcessingTurns;
                    let new_changes = self.process_ai_turn();
                    new_changes.into_iter().for_each(|x| changes.push(x));
                }
            }
        }
        changes
    }

    fn process_ai_turn(&mut self) -> Vec<WorldChange> {
        if let Some(entity_id) = self.turn_manager.current_entity() {
            if let Some(entity) = self.world.get_entity_by_id(&entity_id) {
                if let Some(ai) = entity.ai() {
                    return self.handle_event(ai.get_action(entity_id, &self.world));
                } else {
                    return self.handle_event(GameEvent::SkipTurn);
                }
            }
        }
        vec![]
    }
}

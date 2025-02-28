use std::ops::{Deref, DerefMut};

use crate::{
    application::{events::GameEvent, turns::TurnManager},
    core::types::Direction,
    domain::{
        entity::{self, EntityId},
        world::World,
        world_position::WorldPosition,
    },
};

pub enum ProcessState {
    ProcessingTurns,
    WaitingForPlayer,
}

pub struct GameState {
    pub world: World,
    pub turn_manager: TurnManager,
    pub process_state: ProcessState,
}

impl GameState {
    pub fn new(world: World, turn_manager: TurnManager) -> Self {
        Self {
            world,
            turn_manager,
            process_state: ProcessState::ProcessingTurns,
        }
    }

    pub fn handle_event(&mut self, event: GameEvent) -> () {
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
                        }
                    }
                }
            }
            GameEvent::SkipTurn => {}
        }
    }

    pub fn get_current_entity(&self) -> Option<EntityId> {
        self.turn_manager.current_entity()
    }

    fn progress_turns(&mut self) -> () {
        if let Some(entity_id) = self.turn_manager.next_turn() {
            if let Some(next_entity) = self.world.get_entity_by_id(&entity_id) {
                let is_player_turn = next_entity.id == self.world.player_id;
                if is_player_turn {
                    self.process_state = ProcessState::WaitingForPlayer;
                } else {
                    self.process_state = ProcessState::ProcessingTurns;
                }
            }
        }
    }

    pub fn process_player_action(&mut self, event: GameEvent) -> () {
        self.handle_event(event);
        self.progress_turns();
    }

    fn process_ai_turn(&mut self) -> () {
        if let Some(entity_id) = self.turn_manager.current_entity() {
            if let Some(entity) = self.world.get_entity_by_id(&entity_id) {
                if let Some(ai) = entity.ai() {
                    self.handle_event(ai.get_action(entity_id, &self.world));
                } else {
                    self.handle_event(GameEvent::SkipTurn);
                }
            }
        }
        self.progress_turns();
    }
}

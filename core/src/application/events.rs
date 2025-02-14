// src/application/events.rs
use crate::{
    core::types::Direction,
    domain::{entity::EntityId, world::World, world_position::WorldPosition},
};

#[derive(Debug, Clone)]
pub enum GameEvent {
    MoveByDirection(EntityId, Direction),
    EndTurn(EntityId),
}

impl GameEvent {
    pub fn apply(self, world: &mut World) {
        match self {
            GameEvent::MoveByDirection(entity_id, direction) => {
                let entity = world.get_entity(entity_id);
                let old_pos = match entity {
                    Some(entity) => entity.pos(),
                    None => None,
                };

                let new_pos = match (old_pos, direction) {
                    // if there is no old position, do nothing
                    (None, _) => return,
                    (Some(old_pos), Direction::North) => {
                        WorldPosition::new(old_pos.x, old_pos.y - 1)
                    }
                    (Some(old_pos), Direction::South) => {
                        WorldPosition::new(old_pos.x, old_pos.y + 1)
                    }
                    (Some(old_pos), Direction::East) => {
                        WorldPosition::new(old_pos.x + 1, old_pos.y)
                    }
                    (Some(old_pos), Direction::West) => {
                        WorldPosition::new(old_pos.x - 1, old_pos.y)
                    }
                };

                let entities_in_pos = world.get_entities_by_pos(&new_pos);
                let can_move = entity.is_some_and(|e| e.can_move_to(&new_pos, entities_in_pos));

                match (&can_move, world.get_entity_mut(entity_id)) {
                    (true, Some(entity_mut)) => entity_mut.set_pos(Some(new_pos)),
                    (_, None) => println!("entity #{} not found", entity_id),
                    (false, _) => {
                        println!("entity #{} can't move to position #{}", entity_id, new_pos)
                    }
                };
            }
            GameEvent::EndTurn(_) => (), // Turn management handled by game loop
        }
    }
}

use crate::{
    application::{events::GameEvent, turns::TurnManager},
    core::types::Direction,
    domain::{entity::EntityId, world::World},
};

pub struct GameState {
    pub world: World,
    pub turn_manager: TurnManager,
}

impl GameState {
    pub fn new(world: World, turn_manager: TurnManager) -> Self {
        Self { world, turn_manager }
    }

    pub fn handle_event(&mut self, event: GameEvent) -> StateChanges {
        let mut changes = event.get_changes(&self.world);
        
        // Apply the changes to our state
        for change in &changes {
            match change {
                StateChange::EntityMoved { entity_id, to, .. } => {
                    if let Some(entity) = self.world.get_entity_mut(*entity_id) {
                        entity.set_pos(*to);
                    }
                }
                StateChange::TurnEnded { entity_id } => {
                    if let Some(next_id) = self.turn_manager.next_turn() {
                        changes.push(StateChange::TurnStarted { entity_id: next_id });
                        
                        // Handle AI turns
                        if next_id != 0 {
                            // Simple AI just moves north
                            let ai_changes = self.handle_event(
                                GameEvent::MoveByDirection(next_id, Direction::North)
                            );
                            changes.extend(ai_changes);
                            
                            // End AI turn
                            let end_turn_changes = self.handle_event(
                                GameEvent::EndTurn(next_id)
                            );
                            changes.extend(end_turn_changes);
                        }
                    }
                }
                _ => {}
            }
        }
        
        changes
    }

    pub fn get_current_entity(&self) -> Option<EntityId> {
        self.turn_manager.current_entity()
    }
}

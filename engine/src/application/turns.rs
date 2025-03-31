use std::collections::VecDeque;

use crate::domain::entity::EntityId;

pub struct TurnManager {
    turn_queue: VecDeque<EntityId>,
    current: Option<EntityId>,
}

impl TurnManager {
    pub fn new() -> Self {
        Self {
            turn_queue: VecDeque::new(),
            current: None,
        }
    }

    pub fn initialize(&mut self, player_id: EntityId, other_ids: &[EntityId]) {
        self.turn_queue.clear();
        self.turn_queue.push_back(player_id);
        for &id in other_ids {
            if id != player_id {
                self.turn_queue.push_back(id);
            }
        }
        println!("Turn queue initialized: {:?}", self.turn_queue);
        self.current = Some(player_id);
    }

    pub fn add_entity(&mut self, new_id: EntityId) {
        if !self.turn_queue.contains(&new_id) {
            self.turn_queue.push_back(new_id);
            println!("new entity #{} added to turn queue", new_id);
        }
    }

    /// Remove an entity from the queue (e.g. if it dies).
    /// If it is the current entity, `current` is cleared.
    pub fn remove_entity(&mut self, entity_id: EntityId) {
        self.turn_queue.retain(|&id| id != entity_id);
        if self.current == Some(entity_id) {
            self.current = None;
        }
    }

    /// Move to the next entity in the queue, cycling back to the front if needed.
    /// Returns the new current entity, if the queue isn't empty.
    pub fn next_turn(&mut self) -> Option<EntityId> {
        println!("Next turn");
        if self.turn_queue.is_empty() {
            self.current = None;
            return None;
        }

        // If there's no current, pick the front of the queue
        if self.current.is_none() {
            self.current = self.turn_queue.front().copied();
            return self.current;
        }

        // Otherwise, rotate forward by one.
        let front = self.turn_queue.pop_front().unwrap();
        self.turn_queue.push_back(front);
        self.current = self.turn_queue.front().copied();
        println!("It is now Entity #{}'s turn!", self.current.unwrap());
        self.current
    }

    /// Get the current entity who is acting.
    pub fn current_entity(&self) -> Option<EntityId> {
        self.current
    }
}

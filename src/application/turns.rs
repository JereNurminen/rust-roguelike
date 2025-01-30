use std::collections::VecDeque;

use crate::domain::entity::EntityId;

/// Manages the order in which entities take turns.
pub struct TurnManager {
    /// The ordered queue of entity IDs that will take turns.
    turn_queue: VecDeque<EntityId>,
    /// The entity currently taking its turn (if any).
    current: Option<EntityId>,
}

impl TurnManager {
    pub fn new() -> Self {
        Self {
            turn_queue: VecDeque::new(),
            current: None,
        }
    }

    /// Initialize the queue with given IDs, ensuring the player is first.
    /// For simplicity, we assume `player_id` must always be the first entity.
    pub fn initialize(&mut self, player_id: EntityId, other_ids: &[EntityId]) {
        self.turn_queue.clear();
        // Put the player first
        self.turn_queue.push_back(player_id);
        // Then add any other entity IDs
        for &id in other_ids {
            if id != player_id {
                self.turn_queue.push_back(id);
            }
        }
        self.current = None;
    }

    /// Add a new entity to the end of the queue.
    pub fn add_entity(&mut self, new_id: EntityId) {
        // Avoid duplicates if needed, or assume no duplicates are inserted
        if !self.turn_queue.contains(&new_id) {
            self.turn_queue.push_back(new_id);
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

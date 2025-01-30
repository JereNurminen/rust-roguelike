use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};

use crate::{
    application::{
        events::{process_event, GameEvent},
        turns::TurnManager,
    },
    domain::{entity::EntityId, world::World},
};

/// This struct holds references to the global game state
/// and the turn manager, plus the queues for event processing.
pub struct GameLoop {
    pub world: Arc<Mutex<World>>,
    pub turn_manager: Arc<Mutex<TurnManager>>,
    pub event_receiver: Receiver<GameEvent>,
    pub event_sender: Sender<GameEvent>,
}

impl GameLoop {
    /// The main loop that processes events *and* manages turns.
    /// In a real game, you might call `next_turn()` once at startup.
    pub fn run(&self) {
        // Optionally, start the first turn if the turn manager has a known queue:
        self.start_first_turn();

        // Process incoming events forever.
        while let Ok(event) = self.event_receiver.recv() {
            // 1. Handle special turn-related events:
            println!("Received event {:?}", event);
            match &event {
                GameEvent::EndTurn(entity_id) => {
                    // We'll do the actual end-turn logic after we
                    // apply the domain effect (though here it's a no-op).
                    let next = self.next_turn();
                    // You could also broadcast a "StartTurn(next_id)" event if you wish
                    if let Some(next_id) = next {
                        println!("It is now Entity #{}'s turn!", next_id);
                    }
                }
                _ => {}
            }

            // 2. Apply the domain effect of the event to the World:
            let mut world_guard = self.world.lock().unwrap();
            let follow_ups = process_event(&mut *world_guard, &event);

            drop(world_guard);

            // 4. Spawn follow-up events:
            for e in follow_ups {
                if let Err(err) = self.event_sender.send(e) {
                    eprintln!("Error sending follow-up event: {:?}", err);
                }
            }
        }
    }

    /// Start the first turn if there's nobody "current" yet.
    fn start_first_turn(&self) {
        let mut tm = self.turn_manager.lock().unwrap();
        if tm.current_entity().is_none() {
            let next = tm.next_turn();
            if let Some(id) = next {
                println!("Starting the first turn with Entity #{}!", id);
            }
        }
    }

    /// Move to the next entity in the turn queue.
    /// Returns the new entity's ID, or None if queue is empty.
    fn next_turn(&self) -> Option<EntityId> {
        let mut tm = self.turn_manager.lock().unwrap();
        tm.next_turn()
    }
}

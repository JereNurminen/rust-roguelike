use std::sync::{mpsc::{Receiver, Sender}, Arc, Mutex};
use crate::{
    application::{events::GameEvent, turns::TurnManager},
    core::types::Direction,
    domain::{entity::EntityId, world::World},
};

pub struct GameLoop {
    world: Arc<Mutex<World>>,
    turn_manager: Arc<Mutex<TurnManager>>,
    event_receiver: Receiver<GameEvent>,
    event_sender: Sender<GameEvent>,
}

impl GameLoop {
    pub fn new(
        world: Arc<Mutex<World>>,
        turn_manager: Arc<Mutex<TurnManager>>,
        event_receiver: Receiver<GameEvent>,
        event_sender: Sender<GameEvent>,
    ) -> Self {
        Self {
            world,
            turn_manager,
            event_receiver,
            event_sender,
        }
    }

    pub fn run(&self) {
        self.initialize_turns();
        
        while let Ok(event) = self.event_receiver.recv() {
            let mut world = self.world.lock().unwrap();
            event.apply(&mut world);
            drop(world);

            if let GameEvent::EndTurn(_) = event {
                self.handle_next_turn();
            }
        }
    }

    fn initialize_turns(&self) {
        let mut tm = self.turn_manager.lock().unwrap();
        if tm.current_entity().is_none() {
            if let Some(id) = tm.next_turn() {
                println!("Starting first turn with Entity #{}", id);
            }
        }
    }

    fn handle_next_turn(&self) {
        let next_id = {
            let mut tm = self.turn_manager.lock().unwrap();
            tm.next_turn()
        };

        if let Some(id) = next_id {
            println!("Starting turn for Entity #{}", id);
            if id != 0 { // Non-player entity
                self.run_ai_turn(id);
            }
        }
    }

    fn run_ai_turn(&self, entity_id: EntityId) {
        // Simple AI just moves north and ends turn
        let _ = self.event_sender.send(GameEvent::Move(entity_id, Direction::North));
        let _ = self.event_sender.send(GameEvent::EndTurn(entity_id));
    }
}

use crate::systems::event::Event;
use crate::world::entity::EntityId;
use crate::world::World;

pub struct TurnManager {
    // Which entities still need a turn this round
    turn_queue: Vec<EntityId>,
    current_entity_index: usize,

    // All events produced during an entity’s turn
    event_queue: Vec<Event>,

    // Simple state machine to know what we’re doing
    state: TurnState,
}

#[derive(Debug)]
enum TurnState {
    Idle,
    // We are ready for whichever entity is next in turn_queue
    ReadyForNextEntity,
    // We are processing events that an entity has produced
    ProcessingEvents,
    // We have an animation we must wait on
    WaitingForAnimation,
    // All entities have taken a turn; we’re about to start a fresh round
    EndOfRound,
}

impl TurnManager {
    pub fn new(initial_entities: Vec<EntityId>) -> Self {
        Self {
            turn_queue: initial_entities,
            current_entity_index: 0,
            event_queue: Vec::new(),
            state: TurnState::Idle,
        }
    }

    pub fn start_new_round(&mut self) {
        self.current_entity_index = 0;
        self.state = TurnState::ReadyForNextEntity;
    }

    pub fn update(&mut self, dt: f32, world: &mut World) {
        match self.state {
            TurnState::Idle => {
                // Typically, we do nothing in Idle, or
                // automatically begin the first round.
                self.start_new_round();
            }

            TurnState::ReadyForNextEntity => {
                if self.current_entity_index >= self.turn_queue.len() {
                    // We've processed all entities for this round
                    self.state = TurnState::EndOfRound;
                } else {
                    let entity_id = self.turn_queue[self.current_entity_index];
                    if let Some(events) = self.take_turn(entity_id, world) {
                        // Put all events from entity’s turn in our queue
                        self.event_queue.extend(events);
                        self.state = TurnState::ProcessingEvents;
                    } else {
                        // That entity had no events; move on
                        self.current_entity_index += 1;
                        // Next update cycle will pick next entity
                    }
                }
            }

            TurnState::ProcessingEvents => {
                if !self.event_queue.is_empty() {
                    // Clone or copy the event so we don't keep a reference
                    let event = self.event_queue[0].clone();
                    // Process only the next event
                    let maybe_animation = self.process_event(&event, world);

                    // If the event triggers a blocking animation,
                    // we switch to WaitingForAnimation
                    if maybe_animation {
                        self.state = TurnState::WaitingForAnimation;
                    } else {
                        // No blocking animation, event is done
                        self.event_queue.remove(0);
                    }
                } else {
                    // No more events to process from this entity
                    self.state = TurnState::ReadyForNextEntity;
                    self.current_entity_index += 1;
                }
            }

            TurnState::WaitingForAnimation => {
                // Check if the animation is complete.  This might
                // require checking your World for any “active” animations
                // or a dedicated animation manager system
                if self.animations_done(world) {
                    // Once done, drop the event from queue
                    self.event_queue.remove(0);
                    self.state = TurnState::ProcessingEvents;
                }
            }

            TurnState::EndOfRound => {
                // Possibly prune dead or removed entities,
                // add newly spawned ones, reorder queue, etc.
                self.sync_turn_queue(world);

                // Start new round
                self.start_new_round();
            }
        }
    }

    /// Called when it’s an entity’s turn to act.
    /// Return either None (no events) or Some(vec_of_events)
    fn take_turn(&self, entity_id: EntityId, world: &mut World) -> Option<Vec<Event>> {
        // 1) Check if the entity is still valid.
        // 2) Possibly call an AI function or a "perform_action" method on the entity.
        // 3) Return whatever events that action produces.

        if world.get_entity_by_id(&entity_id).is_none() {
            // entity doesn't exist any more
            return None;
        }

        // If the entity chooses not to act, return None or Some(vec![])
        // For example:
        let events = vec![
            // Example event:
            // Event::EntityAction { actor: entity_id, target: 123, damage: 10 },
        ];
        if events.is_empty() {
            None
        } else {
            Some(events)
        }
    }

    /// Process (apply) the given event to the world. Returns true
    /// if the event triggers a blocking animation we must wait for.
    fn process_event(&mut self, event: &Event, world: &mut World) -> bool {
        match event {
            Event::Move { entity, direction } => {
                let move_info = world.get_entity_by_id(entity).and_then(|entity| {
                    entity
                        .get_position_in_direction(*direction)
                        .filter(|target_pos| world.can_move_to(&entity, target_pos))
                        .map(|target_pos| (entity.id, target_pos))
                });

                if let Some((entity_id, target_pos)) = move_info {
                    world.move_entity(&entity_id, &target_pos);
                }
                false
            }
            _ => todo!("Implement handling for rest of events"),
        }
    }

    fn animations_done(&self, world: &World) -> bool {
        // Check if all your "blocking" animations have completed
        // e.g., query the world’s animation manager.
        true
    }

    fn sync_turn_queue(&mut self, world: &World) {
        // Remove dead or missing entity IDs
        self.turn_queue
            .retain(|id| world.get_entity_by_id(id).is_some());
        // Add newly spawned entities if you have logic to do so:
        // e.g. self.turn_queue.insert(0, new_entity_id);
    }
}

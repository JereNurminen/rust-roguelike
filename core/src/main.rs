// src/main.rs
pub mod application;
pub mod core;
pub mod debug_data;
pub mod domain;
pub mod prefabs;

use application::{
    events::GameEvent,
    game_loop::GameState,
    state_changes::{StateChange, StateChanges},
    turns::TurnManager,
};
use domain::{
    entity::{CoreAttributes, Entity, EntityKind, EntityId, Exhaustion, Status},
    world::World,
    world_position::WorldPosition,
};

pub fn create_initial_game_state() -> GameState {
    let mut world = World::new();
    
    // Add player
    let player_id = world.get_next_entity_id();
    world.add_entity(Entity::new(
        player_id,
        EntityKind::Player,
        Some(WorldPosition::new(0, 0)),
        CoreAttributes::default(),
        Status {
            health: 10,
            stamina: 10,
            mana: 10,
            exhaustion: Exhaustion::Rested,
        },
    ));

    // Add goblin
    let goblin_id = world.get_next_entity_id();
    world.add_entity(
        prefabs::goblins::create_goblin(Some(WorldPosition::new(5, 0))).with_id(goblin_id),
    );

    // Add level
    for entity in debug_data::basic_level::get_level() {
        let id = world.get_next_entity_id();
        world.add_entity(entity.with_id(id));
    }

    // Set up turn manager
    let mut turn_manager = TurnManager::new();
    turn_manager.initialize(player_id, &[goblin_id]);

    GameState::new(world, turn_manager)
}

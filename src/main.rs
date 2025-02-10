// src/main.rs

use macroquad::prelude::*;
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};

mod application;
mod core;
mod debug_data;
mod domain;
mod prefabs;
mod ui; // If needed

use application::{events::GameEvent, game_loop::GameLoop, turns::TurnManager};
use domain::{
    entity::{CoreAttributes, Entity, EntityKind, Exhaustion, SpeciesKind, Status},
    world::World,
    world_position::WorldPosition,
};
use ui::macroquad_adapter::MacroquadUI;

#[macroquad::main("Roguelike")]
async fn main() {
    // Create the world
    let mut initial_world = World::new();

    let player_id = initial_world.get_next_entity_id();
    // Add a player
    initial_world.add_entity(Entity::new(
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

    // Add a goblin
    let goblin_id = initial_world.get_next_entity_id();
    initial_world.add_entity(
        prefabs::goblins::create_goblin(Some(WorldPosition::new(5, 0))).with_id(goblin_id),
    );

    for (_, entity) in debug_data::basic_level::get_level().into_iter().enumerate() {
        let id = initial_world.get_next_entity_id();
        initial_world.add_entity(entity.with_id(id));
    }

    let shared_world = Arc::new(Mutex::new(initial_world));

    // Set up the TurnManager
    let mut turn_manager = TurnManager::new();
    // We know about two entities: player #0 and goblin #1.
    turn_manager.initialize(0, &[1]);
    let shared_tm = Arc::new(Mutex::new(turn_manager));

    // Set up channels for events
    let (event_sender, event_receiver): (Sender<_>, Receiver<_>) = channel();

    // Build the application-side game loop
    let game_loop = GameLoop::new(
        shared_world.clone(),
        shared_tm.clone(),
        event_receiver,
        event_sender.clone(),
    );

    // Spawn a thread to run the turn/event processing
    std::thread::spawn(move || {
        game_loop.run();
    });

    // Construct our UI struct
    let mut ui = MacroquadUI::new(
        shared_world.clone(),
        event_sender.clone(),
        shared_tm.clone(),
    );

    // Our "main" loop for Macroquad is the rendering loop:
    loop {
        clear_background(BLACK);

        // Handle input
        ui.handle_input();
        ui.handle_player_input(0);

        // Draw world
        ui.draw_grid();
        ui.draw_world();
        ui.highlight_selected_tile();

        next_frame().await;
    }
}

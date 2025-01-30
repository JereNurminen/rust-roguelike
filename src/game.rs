use std::time::Instant;

use crate::ui::{MenuType, UiState};
use crate::world::entity::{DisplayProperties, Sprite};
use crate::world::{
    entity::{CoreAttributes, EntityId, Exhaustion, Status},
    Entity, World, WorldPosition,
};
use macroquad::prelude::*;

pub enum GameState {
    PlayerTurn,
    ProcessingWorldTurns,
    AnimatingTurnTransition { pending_animations: Vec<usize> },
    InMenu(MenuType),
}

pub struct Game {
    world: World,
    player_entity_id: EntityId,
    ui: UiState,
    state: GameState,
    last_update: Instant,
}

impl Game {
    pub fn new() -> Self {
        let player = Entity::new(
            0,
            crate::world::entity::EntityKind::Player,
            Some(WorldPosition { x: 0, y: 0 }),
            CoreAttributes::default(),
            Status {
                health: 10,
                stamina: 10,
                mana: 10,
                exhaustion: Exhaustion::Rested,
            },
            Some(DisplayProperties {
                visual_position: Vec2::new(0.0, 0.0),
                sprite: Sprite::Static(Image::gen_image_color(
                    16,
                    16,
                    Color::from_rgba(255, 0, 0, 255),
                )),
            }),
        );

        Self {
            world: World::new(vec![player]),
            player_entity_id: 0,
            ui: UiState::new(),
            state: GameState::PlayerTurn,
            last_update: Instant::now(),
        }
    }

    pub async fn run(&mut self) {
        loop {
            self.update();
            self.render();
            next_frame().await;
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // Update UI state (handle input)
        self.ui.update();

        // Update world
        self.world.update(dt);

        // Handle game state transitions
        /*
        match &mut self.state {
            GameState::AnimatingTurnTransition { pending_animations } => {
                pending_animations.retain(|&entity_id| self.world.is_entity_animating(entity_id));
                if pending_animations.is_empty() {
                    self.state = GameState::PlayerTurn;
                }
            }
            _ => {}
        }
        */

        // Handle input for game state changes
        if is_key_pressed(KeyCode::Escape) {
            match self.state {
                GameState::InMenu(_) => self.state = GameState::PlayerTurn,
                GameState::PlayerTurn => self.state = GameState::InMenu(MenuType::Pause),
                _ => {}
            }
        }

        // Add other input handling here
        if is_key_pressed(KeyCode::I) && matches!(self.state, GameState::PlayerTurn) {
            self.state = GameState::InMenu(MenuType::Inventory);
        }
        if is_key_pressed(KeyCode::C) && matches!(self.state, GameState::PlayerTurn) {
            self.state = GameState::InMenu(MenuType::Character);
        }
    }

    fn render(&mut self) {
        clear_background(BLACK);

        // Render game world
        self.ui.render_game(&self.world);

        // Render active menu if in menu state
        if let GameState::InMenu(menu_type) = &self.state {
            self.ui.render_menu(menu_type);
        }
    }
}

// Main game entry point
#[macroquad::main("Roguelike")]
async fn main() {
    let mut game = Game::new();
    game.run().await;
}

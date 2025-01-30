use macroquad::prelude::*;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use crate::core::types::Direction;
use crate::domain::entity::EntityKind;
use crate::domain::world::World;
use crate::domain::world_position::WorldPosition;
use crate::{
    application::events::GameEvent, application::turns::TurnManager, domain::entity::EntityId,
};

use super::camera::{Camera, TILE_SIZE};
use super::input_handler::InputHandler;

pub struct MacroquadUI {
    pub world: Arc<Mutex<World>>,
    pub event_sender: Sender<GameEvent>,
    pub turn_manager: Arc<Mutex<TurnManager>>,
    pub camera: Camera,
    pub input: InputHandler,
    pub selected_tile: Option<WorldPosition>,
}

impl MacroquadUI {
    pub fn new(
        world: Arc<Mutex<World>>,
        event_sender: Sender<GameEvent>,
        turn_manager: Arc<Mutex<TurnManager>>,
    ) -> Self {
        Self {
            world,
            event_sender,
            turn_manager: turn_manager,
            camera: Camera::new(),
            input: InputHandler::new(),
            selected_tile: None,
        }
    }

    pub fn handle_tile_selection(&mut self) {
        if let Some(clicked_tile) = self.input.handle_input(&mut self.camera) {
            // We store the selection for later use (e.g., drawing highlight).
            self.selected_tile = Some(clicked_tile);
            println!("Tile selected: {:?}", clicked_tile);
        }
    }

    /// Called every frame in the main loop to handle user input for camera.
    /// If the user clicks on a tile, we might generate a domain event or highlight something.
    pub fn handle_camera_input(&mut self) {
        if let Some(clicked_tile) = self.input.handle_input(&mut self.camera) {
            // Possibly do something with `clicked_tile`,
            // e.g. send a domain event or store a selection in the UI.
            println!("User clicked tile: {:?}", clicked_tile);

            // For example, you might want to send a "SelectTile" event:
            // let _ = self.event_sender.send(GameEvent::SelectTile(clicked_tile.x, clicked_tile.y));
        }
    }

    /// Handle user input for the player's turn
    pub fn handle_player_input(&mut self, player_id: EntityId) {
        let tm = self.turn_manager.lock().unwrap();
        if tm.current_entity() != Some(player_id) {
            // It's not the player's turn, so ignore input
            return;
        }
        drop(tm);

        let direction = if is_key_pressed(KeyCode::L) {
            Some(Direction::East)
        } else if is_key_pressed(KeyCode::H) {
            Some(Direction::West)
        } else if is_key_pressed(KeyCode::K) {
            Some(Direction::North)
        } else if is_key_pressed(KeyCode::J) {
            Some(Direction::South)
        } else {
            None
        };

        if let Some(direction) = direction {
            let _ = self
                .event_sender
                .send(GameEvent::Move(player_id, direction));
        }
    }

    /// Draw the current state of the world
    pub fn draw_world(&self) {
        let world_guard = self.world.lock().unwrap();
        for entity in world_guard.entities.values() {
            if let Some(pos) = entity.pos() {
                let screen_pos = self.camera.world_to_screen(pos);

                let color = if *entity.kind() == EntityKind::Player {
                    GREEN
                } else {
                    RED
                };

                draw_circle(screen_pos.x, screen_pos.y, 10.0, color);
            }
        }
    }

    /// Draw a grid of lines in the visible region of the camera.
    /// This uses `tile_size` as the (unzoomed) screen space for each tile.
    /// If you want the lines to follow the exact camera zoom, adapt accordingly.
    pub fn draw_grid(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Convert screen corners to world coords:
        let top_left = self.camera.screen_to_world(Vec2::new(0.0, 0.0));
        let bottom_right = self.camera.screen_to_world(Vec2::new(screen_w, screen_h));

        // We extend one tile out in each direction, so the grid lines fill the screen
        let start_x = top_left.x - 1;
        let end_x = bottom_right.x + 1;
        let start_y = top_left.y - 1;
        let end_y = bottom_right.y + 1;

        // Vertical lines
        for x in start_x..=end_x {
            let start_pos = self.camera.world_to_screen(WorldPosition::new(x, start_y));
            let end_pos = self.camera.world_to_screen(WorldPosition::new(x, end_y));
            draw_line(
                start_pos.x - TILE_SIZE * self.camera.zoom / 2.0,
                start_pos.y - TILE_SIZE * self.camera.zoom / 2.0,
                end_pos.x - TILE_SIZE * self.camera.zoom / 2.0,
                end_pos.y - TILE_SIZE * self.camera.zoom / 2.0,
                1.0,
                Color::new(0.3, 0.3, 0.3, 1.0),
            );
        }

        // Horizontal lines
        for y in start_y..=end_y {
            let start_pos = self.camera.world_to_screen(WorldPosition::new(start_x, y));
            let end_pos = self.camera.world_to_screen(WorldPosition::new(end_x, y));
            draw_line(
                start_pos.x - TILE_SIZE * self.camera.zoom / 2.0,
                start_pos.y - TILE_SIZE * self.camera.zoom / 2.0,
                end_pos.x - TILE_SIZE * self.camera.zoom / 2.0,
                end_pos.y - TILE_SIZE * self.camera.zoom / 2.0,
                1.0,
                Color::new(0.3, 0.3, 0.3, 1.0),
            );
        }
    }

    pub fn highlight_selected_tile(&self) {
        if let Some(pos) = self.selected_tile {
            let screen_pos = self.camera.world_to_screen(pos);
            draw_rectangle_lines(
                screen_pos.x - TILE_SIZE / 2.0,
                screen_pos.y - TILE_SIZE / 2.0,
                TILE_SIZE,
                TILE_SIZE,
                2.0,
                YELLOW,
            );
        }
    }
}

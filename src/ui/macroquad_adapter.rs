use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use crate::core::types::Direction;
use crate::domain::entity::{EntityId, EntityKind};
use crate::domain::world::World;
use crate::domain::world_position::WorldPosition;
use crate::{application::events::GameEvent, application::turns::TurnManager};

use super::camera::{Camera, TILE_SIZE};
use super::input_handler::InputHandler;

pub struct MacroquadUI {
    pub world: Arc<Mutex<World>>,
    pub event_sender: Sender<GameEvent>,
    pub turn_manager: Arc<Mutex<TurnManager>>,
    pub camera: Camera,
    pub input: InputHandler,
    pub selected_tile: Option<WorldPosition>,
    pub mouse_position: Vec2,
    pub selected_entity_id: Option<EntityId>,
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
            turn_manager,
            camera: Camera::new(),
            input: InputHandler::new(),
            selected_tile: None,
            mouse_position: Vec2::ZERO,
            selected_entity_id: None,
        }
    }

    pub fn handle_input(&mut self) {
        self.mouse_position = Vec2::from(mouse_position());
        if let Some(clicked_tile) = self.input.handle_input(&mut self.camera) {
            let world = self.world.lock().unwrap();
            let entities = world.get_entities_by_pos(&clicked_tile);

            if !entities.is_empty() {
                self.selected_tile = Some(clicked_tile);
                // Just take the first entity for now
                self.selected_entity_id = Some(entities[0].id);
            } else {
                self.selected_tile = None;
                self.selected_entity_id = None;
            }
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
            self.event_sender
                .send(GameEvent::MoveByDirection(player_id, direction))
                .expect("Panic while moving player");
            self.event_sender
                .send(GameEvent::EndTurn(player_id))
                .expect("Panic while ending player's turn");
        }
    }

    /// Draw the current state of the world
    pub fn draw_world(&self) {
        let world_guard = self.world.lock().unwrap();
        for entity in world_guard.entities.values() {
            if let Some(pos) = entity.pos() {
                let screen_pos = self.camera.world_to_screen(pos);

                let color = match entity.kind() {
                    EntityKind::Player => GREEN,
                    EntityKind::Npc { .. } => RED,
                    EntityKind::Wall { .. } => GRAY,
                    _ => WHITE,
                };
                draw_circle(screen_pos.x, screen_pos.y, 10.0 * self.camera.zoom, color);
            }
        }
        drop(world_guard);
        self.draw_popup();
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
            let start_world = WorldPosition::new(x, start_y);
            let end_world = WorldPosition::new(x, end_y);
            let start_pos = self.camera.world_to_screen(start_world);
            let end_pos = self.camera.world_to_screen(end_world);
            draw_line(
                start_pos.x,
                start_pos.y,
                end_pos.x,
                end_pos.y,
                1.0,
                Color::new(0.3, 0.3, 0.3, 1.0),
            );
        }

        // Horizontal lines
        for y in start_y..=end_y {
            let start_world = WorldPosition::new(start_x, y);
            let end_world = WorldPosition::new(end_x, y);
            let start_pos = self.camera.world_to_screen(start_world);
            let end_pos = self.camera.world_to_screen(end_world);
            draw_line(
                start_pos.x,
                start_pos.y,
                end_pos.x,
                end_pos.y,
                1.0,
                Color::new(0.3, 0.3, 0.3, 1.0),
            );
        }
    }

    pub fn highlight_selected_tile(&self) {
        if let Some(pos) = self.selected_tile {
            let screen_pos = self.camera.world_to_screen(pos);
            let size = TILE_SIZE * self.camera.zoom;
            draw_rectangle_lines(
                screen_pos.x - size / 2.0,
                screen_pos.y - size / 2.0,
                size,
                size,
                2.0,
                YELLOW,
            );
        }
    }

    fn draw_popup(&self) {
        if let Some(entity_id) = self.selected_entity_id {
            let world = self.world.lock().unwrap();
            if let Some(entity) = world.get_entity(entity_id) {
                if let Some(pos) = entity.pos() {
                    let screen_pos = self.camera.world_to_screen(pos);

                    // Popup dimensions
                    let popup_width = 200.0;
                    let popup_height = 100.0;

                    // Position to the right of the entity
                    let popup_x = screen_pos.x + 20.0;
                    let popup_y = screen_pos.y;

                    root_ui().window(
                        hash!(entity_id + popup_x as usize + popup_y as usize),
                        Vec2::new(popup_x, popup_y),
                        Vec2::new(popup_width, popup_height),
                        |ui| {
                            let description = match entity.kind() {
                                EntityKind::Player => "Player".to_string(),
                                EntityKind::Npc { species } => format!("{:?}", species),
                                EntityKind::Wall { .. } => "Wall".to_string(),
                                EntityKind::Floor { .. } => "Floor".to_string(),
                                _ => "Unknown".to_string(),
                            };
                            ui.label(None, &description);
                        },
                    );
                }
            }
        }
    }
}

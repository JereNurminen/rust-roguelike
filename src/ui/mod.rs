use crate::world::entity::EntityKind;
use crate::world::{World, WorldPosition};
use macroquad::prelude::*;

pub struct UiState {
    camera: Camera,
    selected_tile: Option<WorldPosition>,
    drag_start: Option<Vec2>,
    initial_click_pos: Option<Vec2>,
}

pub struct Camera {
    pos: WorldPosition,
    zoom: f32,
    drag_offset: Vec2,
}

pub enum MenuType {
    Inventory,
    Character,
    Pause,
}

pub const TILE_SIZE: f32 = 32.0;

impl UiState {
    pub fn new() -> Self {
        Self {
            camera: Camera {
                pos: WorldPosition::new(0, 0),
                zoom: 1.0,
                drag_offset: Vec2::ZERO,
            },
            selected_tile: None,
            drag_start: None,
            initial_click_pos: None, // Initialize new field
        }
    }

    pub fn update(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let current_pos: Vec2 = mouse_position().into();
            self.drag_start = Some(current_pos);
            self.initial_click_pos = Some(current_pos); // Store initial click position
            self.camera.drag_offset = Vec2::ZERO;
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(start) = self.drag_start {
                let current: Vec2 = mouse_position().into();
                let delta = current - start;

                // Convert screen space delta to world space delta
                let world_delta = delta / (TILE_SIZE * self.camera.zoom);

                // Accumulate the movement
                self.camera.drag_offset += world_delta;

                // Update camera position when accumulated movement is large enough
                let dx = self.camera.drag_offset.x.floor() as i32;
                let dy = self.camera.drag_offset.y.floor() as i32;

                if dx != 0 || dy != 0 {
                    self.camera.pos.x -= dx;
                    self.camera.pos.y -= dy;

                    // Subtract the applied movement from the accumulator
                    self.camera.drag_offset.x -= dx as f32;
                    self.camera.drag_offset.y -= dy as f32;
                }

                // Update drag start for next frame
                self.drag_start = Some(current);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let Some(initial_pos) = self.initial_click_pos {
                // Use initial click position
                let current: Vec2 = mouse_position().into();
                let delta = current - initial_pos; // Compare with initial position

                // If movement was small enough, treat as a click
                if delta.length() < 5.0 {
                    self.selected_tile = Some(self.screen_to_world(current));
                }
            }
            self.drag_start = None;
            self.initial_click_pos = None; // Clear initial position
        }

        // Handle zoom
        let wheel = mouse_wheel().1;
        if wheel != 0.0 {
            let mouse_pos: Vec2 = mouse_position().into();
            let before_zoom = self.screen_to_world(mouse_pos);

            let old_zoom = self.camera.zoom;
            self.camera.zoom = (self.camera.zoom * (1.0 + wheel * 0.1)).clamp(0.1, 3.0);

            if old_zoom != self.camera.zoom {
                let after_zoom = self.screen_to_world(mouse_pos);
                self.camera.pos.x += before_zoom.x - after_zoom.x;
                self.camera.pos.y += before_zoom.y - after_zoom.y;
            }
        }
    }

    pub fn render_game(&mut self, world: &World) {
        clear_background(BLACK);

        let tile_size = TILE_SIZE * self.camera.zoom;

        self.draw_grid(tile_size);

        for entity in world.entities().values() {
            if let Some(position) = entity.position() {
                let screen_pos = self.world_to_screen(position);

                match entity.kind() {
                    EntityKind::Floor { .. } => {
                        draw_rectangle(
                            screen_pos.x - tile_size / 2.0,
                            screen_pos.y - tile_size / 2.0,
                            tile_size,
                            tile_size,
                            DARKGRAY,
                        );
                    }
                    EntityKind::Wall { .. } => {
                        draw_rectangle(
                            screen_pos.x - tile_size / 2.0,
                            screen_pos.y - tile_size / 2.0,
                            tile_size,
                            tile_size,
                            GRAY,
                        );
                    }
                    EntityKind::Npc { .. } => {
                        draw_circle(screen_pos.x, screen_pos.y, tile_size / 2.0, RED);
                        draw_text(
                            "X",
                            screen_pos.x - tile_size / 4.0,
                            screen_pos.y + tile_size / 4.0,
                            tile_size,
                            WHITE,
                        );
                    }
                    EntityKind::Player => {
                        draw_circle(screen_pos.x, screen_pos.y, tile_size / 2.0, YELLOW);
                    }
                    _ => {}
                }
            }
        }

        // Render selection
        if let Some(pos) = self.selected_tile {
            let screen_pos = self.world_to_screen(pos);
            draw_rectangle_lines(
                screen_pos.x - tile_size / 2.0,
                screen_pos.y - tile_size / 2.0,
                tile_size,
                tile_size,
                2.0,
                YELLOW,
            );
        }
    }

    pub fn render_menu(&self, menu_type: &MenuType) {
        match menu_type {
            MenuType::Inventory => {
                // Draw inventory menu using Macroquad drawing functions
                let window_width = 200.0;
                let window_height = 300.0;
                draw_rectangle(
                    screen_width() / 2.0 - window_width / 2.0,
                    screen_height() / 2.0 - window_height / 2.0,
                    window_width,
                    window_height,
                    Color::new(0.2, 0.2, 0.2, 0.9),
                );
                draw_text(
                    "Inventory",
                    screen_width() / 2.0 - 40.0,
                    screen_height() / 2.0 - window_height / 2.0 + 30.0,
                    30.0,
                    WHITE,
                );
            }
            MenuType::Character => {
                // Draw character menu
                // Similar to inventory menu but with different content
            }
            MenuType::Pause => {
                // Draw pause menu
                // Similar to inventory menu but with different content
            }
        }
    }

    fn world_to_screen(&self, pos: WorldPosition) -> Vec2 {
        let offset = Vec2::new(
            self.camera.pos.x as f32 * TILE_SIZE * self.camera.zoom,
            self.camera.pos.y as f32 * TILE_SIZE * self.camera.zoom,
        );
        Vec2::new(
            pos.x as f32 * TILE_SIZE * self.camera.zoom - offset.x + screen_width() / 2.0,
            pos.y as f32 * TILE_SIZE * self.camera.zoom - offset.y + screen_height() / 2.0,
        )
    }

    fn screen_to_world(&self, screen_pos: Vec2) -> WorldPosition {
        let screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        let relative_pos = screen_pos - screen_center;
        let world_pos = (relative_pos
            + Vec2::new(
                TILE_SIZE * self.camera.zoom / 2.0,
                TILE_SIZE * self.camera.zoom / 2.0,
            ))
            / (TILE_SIZE * self.camera.zoom);

        WorldPosition::new(
            (world_pos.x + self.camera.pos.x as f32).floor() as i32,
            (world_pos.y + self.camera.pos.y as f32).floor() as i32,
        )
    }

    fn draw_grid(&self, tile_size: f32) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Calculate visible range in world coordinates
        let top_left = self.screen_to_world(Vec2::new(0.0, 0.0));
        let bottom_right = self.screen_to_world(Vec2::new(screen_w, screen_h));

        // Add some padding to ensure we draw enough grid lines
        let start_x = top_left.x - 1;
        let end_x = bottom_right.x + 1;
        let start_y = top_left.y - 1;
        let end_y = bottom_right.y + 1;

        // Draw vertical lines
        for x in start_x..=end_x {
            let start_pos = self.world_to_screen(WorldPosition::new(x, start_y));
            let end_pos = self.world_to_screen(WorldPosition::new(x, end_y));
            draw_line(
                start_pos.x - tile_size / 2.0, // Offset by half a tile
                start_pos.y - tile_size / 2.0, // Offset by half a tile
                end_pos.x - tile_size / 2.0,   // Offset by half a tile
                end_pos.y - tile_size / 2.0,   // Offset by half a tile
                1.0,
                Color::new(0.3, 0.3, 0.3, 1.0),
            );
        }

        // Draw horizontal lines
        for y in start_y..=end_y {
            let start_pos = self.world_to_screen(WorldPosition::new(start_x, y));
            let end_pos = self.world_to_screen(WorldPosition::new(end_x, y));
            draw_line(
                start_pos.x - tile_size / 2.0, // Offset by half a tile
                start_pos.y - tile_size / 2.0, // Offset by half a tile
                end_pos.x - tile_size / 2.0,   // Offset by half a tile
                end_pos.y - tile_size / 2.0,   // Offset by half a tile
                1.0,
                Color::new(0.3, 0.3, 0.3, 1.0),
            );
        }
    }
}

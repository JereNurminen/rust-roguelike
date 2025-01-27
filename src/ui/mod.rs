use crate::world::entity::EntityKind;
use crate::world::{World, WorldPosition};
use macroquad::prelude::*;

pub struct UiState {
    camera: Camera,
    selected_tile: Option<WorldPosition>,
    open_menu: Option<MenuType>,
    drag_start: Option<Vec2>,
}

pub struct Camera {
    pos: WorldPosition,
    zoom: f32,
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
            },
            selected_tile: None,
            open_menu: None,
            drag_start: None,
        }
    }

    pub fn update(&mut self) {
        // Handle camera drag
        if is_mouse_button_pressed(MouseButton::Left) {
            self.drag_start = Some(Vec2::new(mouse_position().0, mouse_position().1));
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(start) = self.drag_start {
                let current = Vec2::new(mouse_position().0, mouse_position().1);
                let delta = current - start;
                self.camera.pos.x -= (delta.x / self.camera.zoom) as i32;
                self.camera.pos.y -= (delta.y / self.camera.zoom) as i32;
                self.drag_start = Some(current);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            // If the mouse hasn't moved much, treat it as a click
            if let Some(start) = self.drag_start {
                let current = Vec2::new(mouse_position().0, mouse_position().1);
                let delta = current - start;
                if delta.length() < 5.0 {
                    self.selected_tile = Some(self.screen_to_world(current));
                }
            }
            self.drag_start = None;
        }

        // Handle zoom
        let wheel = mouse_wheel().1;
        if wheel != 0.0 {
            self.camera.zoom = (self.camera.zoom * (1.0 + wheel * 0.1)).clamp(0.1, 3.0);
        }
    }

    pub fn render_game(&mut self, world: &World) {
        clear_background(BLACK);

        let tile_size = TILE_SIZE * self.camera.zoom;

        // Render entities
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
        let offset = Vec2::new(
            self.camera.pos.x as f32 * TILE_SIZE * self.camera.zoom,
            self.camera.pos.y as f32 * TILE_SIZE * self.camera.zoom,
        );
        WorldPosition::new(
            ((screen_pos.x + offset.x - screen_width() / 2.0) / (TILE_SIZE * self.camera.zoom))
                as i32,
            ((screen_pos.y + offset.y - screen_height() / 2.0) / (TILE_SIZE * self.camera.zoom))
                as i32,
        )
    }
}

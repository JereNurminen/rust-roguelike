use super::camera::Camera;
use super::types::TILE_SIZE;
use crate::world::entity::EntityKind;
use crate::world::{World, WorldPosition};
use macroquad::prelude::*;

pub struct Renderer {
    pub camera: Camera,
}

impl Renderer {
    pub fn new(camera: Camera) -> Self {
        Self { camera }
    }

    pub fn draw_world(&self, world: &World, selected_tile: Option<WorldPosition>) {
        clear_background(BLACK);

        let tile_size = TILE_SIZE * self.camera.zoom;

        self.draw_grid(tile_size);
        self.draw_entities(world, tile_size);
        self.draw_selection(selected_tile, tile_size);
    }

    fn draw_entities(&self, world: &World, tile_size: f32) {
        for entity in world.entities().values() {
            if let Some(position) = entity.position() {
                let screen_pos = self.camera.world_to_screen(position);

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
    }

    fn draw_selection(&self, selected_tile: Option<WorldPosition>, tile_size: f32) {
        if let Some(pos) = selected_tile {
            let screen_pos = self.camera.world_to_screen(pos);
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

    fn draw_grid(&self, tile_size: f32) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        let top_left = self.camera.screen_to_world(Vec2::new(0.0, 0.0));
        let bottom_right = self.camera.screen_to_world(Vec2::new(screen_w, screen_h));

        let start_x = top_left.x - 1;
        let end_x = bottom_right.x + 1;
        let start_y = top_left.y - 1;
        let end_y = bottom_right.y + 1;

        for x in start_x..=end_x {
            let start_pos = self.camera.world_to_screen(WorldPosition::new(x, start_y));
            let end_pos = self.camera.world_to_screen(WorldPosition::new(x, end_y));
            draw_line(
                start_pos.x - tile_size / 2.0,
                start_pos.y - tile_size / 2.0,
                end_pos.x - tile_size / 2.0,
                end_pos.y - tile_size / 2.0,
                1.0,
                Color::new(0.3, 0.3, 0.3, 1.0),
            );
        }

        for y in start_y..=end_y {
            let start_pos = self.camera.world_to_screen(WorldPosition::new(start_x, y));
            let end_pos = self.camera.world_to_screen(WorldPosition::new(end_x, y));
            draw_line(
                start_pos.x - tile_size / 2.0,
                start_pos.y - tile_size / 2.0,
                end_pos.x - tile_size / 2.0,
                end_pos.y - tile_size / 2.0,
                1.0,
                Color::new(0.3, 0.3, 0.3, 1.0),
            );
        }
    }
}

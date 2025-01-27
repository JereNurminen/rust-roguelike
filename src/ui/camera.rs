use super::types::TILE_SIZE;
use crate::world::WorldPosition;
use macroquad::prelude::*;

pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,
    pub drag_offset: Vec2,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
            zoom: 1.0,
            drag_offset: Vec2::ZERO,
        }
    }

    pub fn world_to_screen(&self, pos: WorldPosition) -> Vec2 {
        let offset = Vec2::new(
            self.pos.x * TILE_SIZE * self.zoom,
            self.pos.y * TILE_SIZE * self.zoom,
        );
        Vec2::new(
            pos.x as f32 * TILE_SIZE * self.zoom - offset.x + screen_width() / 2.0,
            pos.y as f32 * TILE_SIZE * self.zoom - offset.y + screen_height() / 2.0,
        )
    }

    pub fn world_to_screen_f(&self, pos: Vec2) -> Vec2 {
        let offset = Vec2::new(
            self.pos.x * TILE_SIZE * self.zoom,
            self.pos.y * TILE_SIZE * self.zoom,
        );
        Vec2::new(
            pos.x * TILE_SIZE * self.zoom - offset.x + screen_width() / 2.0,
            pos.y * TILE_SIZE * self.zoom - offset.y + screen_height() / 2.0,
        )
    }

    pub fn screen_to_world_f(&self, screen_pos: Vec2) -> Vec2 {
        let screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        let relative_pos = screen_pos - screen_center;
        let adjusted_pos =
            relative_pos + Vec2::new(TILE_SIZE * self.zoom / 2.0, TILE_SIZE * self.zoom / 2.0);
        let world_pos = adjusted_pos / (TILE_SIZE * self.zoom);
        world_pos + self.pos
    }

    pub fn screen_to_world(&self, screen_pos: Vec2) -> WorldPosition {
        let world_pos = self.screen_to_world_f(screen_pos);
        WorldPosition::new(world_pos.x.floor() as i32, world_pos.y.floor() as i32)
    }
}

// src/ui/camera.rs
use macroquad::prelude::*;

use crate::domain::world_position::WorldPosition;

/// The size of one tile in screen pixels before zoom.
/// Typically a UI concern (though you might store it in a config).
pub const TILE_SIZE: f32 = 32.0;

pub struct Camera {
    pub pos: Vec2, // camera's center in "world coords" (floats)
    pub zoom: f32, // how much we zoom in/out
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

    /// Convert a world tile position (integer coords) to a screen position (pixels).
    pub fn world_to_screen(&self, pos: WorldPosition) -> Vec2 {
        // We multiply world coords by TILE_SIZE and zoom,
        // then offset by the camera center.
        let offset = Vec2::new(
            self.pos.x * TILE_SIZE * self.zoom,
            self.pos.y * TILE_SIZE * self.zoom,
        );
        Vec2::new(
            pos.x as f32 * TILE_SIZE * self.zoom - offset.x + screen_width() / 2.0,
            pos.y as f32 * TILE_SIZE * self.zoom - offset.y + screen_height() / 2.0,
        )
    }

    /// Same as above, but for floating-point world coords
    /// (if your game supports sub-tile movement).
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

    /// Convert a screen (pixel) position back to floating-point world coords.
    pub fn screen_to_world_f(&self, screen_pos: Vec2) -> Vec2 {
        let screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        let relative_pos = screen_pos - screen_center;
        let adjusted_pos =
            relative_pos + Vec2::new(TILE_SIZE * self.zoom / 2.0, TILE_SIZE * self.zoom / 2.0);
        let world_pos = adjusted_pos / (TILE_SIZE * self.zoom);
        world_pos + self.pos
    }

    /// Convert a screen position to an integer tile coordinate.
    pub fn screen_to_world(&self, screen_pos: Vec2) -> WorldPosition {
        let world_pos = self.screen_to_world_f(screen_pos);
        WorldPosition::new(world_pos.x.floor() as i32, world_pos.y.floor() as i32)
    }
}

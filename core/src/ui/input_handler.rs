use crate::domain::world_position::WorldPosition;

// src/ui/input_handler.rs
use super::camera::Camera;
use macroquad::prelude::*;

pub struct InputHandler {
    drag_start: Option<Vec2>,
    initial_click_pos: Option<Vec2>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            drag_start: None,
            initial_click_pos: None,
        }
    }

    /// Handle panning, zooming, and left-click. Returns an Option<WorldPosition>
    /// if the user clicked on a tile (rather than dragging).
    pub fn handle_input(&mut self, camera: &mut Camera) -> Option<WorldPosition> {
        // Mouse press
        if is_mouse_button_pressed(MouseButton::Left) {
            let current_pos = mouse_position().into();
            self.drag_start = Some(current_pos);
            self.initial_click_pos = Some(current_pos);
        }

        // Mouse drag
        if is_mouse_button_down(MouseButton::Left) {
            if let Some(start) = self.drag_start {
                let current: Vec2 = mouse_position().into();
                let delta = current - start;

                // Move the camera position by the world-space delta
                let world_delta = delta / (super::camera::TILE_SIZE * camera.zoom);
                camera.pos -= world_delta;

                // Update drag_start so that next iteration is relative
                self.drag_start = Some(current);
            }
        }

        // Mouse release
        if is_mouse_button_released(MouseButton::Left) {
            if let Some(initial_pos) = self.initial_click_pos {
                let current: Vec2 = mouse_position().into();
                let delta = current - initial_pos;

                // If the mouse didn't move much, consider it a click
                if delta.length() < 5.0 {
                    // Return the selected tile position
                    let tile_pos = camera.screen_to_world(current);
                    // Clear drag state
                    self.drag_start = None;
                    self.initial_click_pos = None;
                    return Some(tile_pos);
                }
            }

            self.drag_start = None;
            self.initial_click_pos = None;
        }

        // Zoom
        self.handle_zoom(camera);

        None
    }

    fn handle_zoom(&mut self, camera: &mut Camera) {
        let wheel_delta = mouse_wheel().1;
        if wheel_delta != 0.0 {
            let mouse_pos = Vec2::from(mouse_position());
            let before_zoom = camera.screen_to_world_f(mouse_pos);

            let old_zoom = camera.zoom;
            let target_zoom = (camera.zoom * (1.0 + wheel_delta * 0.1)).clamp(0.1, 3.0);

            // You can lerp or instantly set the zoom:
            camera.zoom = target_zoom;

            if old_zoom != camera.zoom {
                // We want the mouse position in the world to remain "under" the cursor
                let after_zoom = camera.screen_to_world_f(mouse_pos);
                camera.pos += before_zoom - after_zoom;
            }
        }
    }
}

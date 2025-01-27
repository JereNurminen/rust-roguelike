use super::camera::Camera;
use super::types::TILE_SIZE;
use crate::world::WorldPosition;
use macroquad::prelude::*;

pub struct InputHandler {
    drag_start: Option<Vec2>,
    initial_click_pos: Option<Vec2>,
    zoom_target: Option<f32>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            drag_start: None,
            initial_click_pos: None,
            zoom_target: None,
        }
    }

    pub fn handle_input(&mut self, camera: &mut Camera) -> Option<WorldPosition> {
        if is_mouse_button_pressed(MouseButton::Left) {
            let current_pos: Vec2 = mouse_position().into();
            self.drag_start = Some(current_pos);
            self.initial_click_pos = Some(current_pos);
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(start) = self.drag_start {
                let current: Vec2 = mouse_position().into();
                let delta = current - start;
                let world_delta = delta / (TILE_SIZE * camera.zoom);
                camera.pos -= world_delta;
                self.drag_start = Some(current);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let Some(initial_pos) = self.initial_click_pos {
                let current: Vec2 = mouse_position().into();
                let delta = current - initial_pos;

                // If the mouse hasn't moved much, consider it a click
                if delta.length() < 5.0 {
                    // Return the selected tile position
                    return Some(camera.screen_to_world(current));
                }
            }
            self.drag_start = None;
            self.initial_click_pos = None;
        }

        self.handle_zoom(camera);

        None
    }

    fn handle_zoom(&mut self, camera: &mut Camera) {
        let wheel = mouse_wheel().1;
        if wheel != 0.0 {
            let mouse_pos: Vec2 = mouse_position().into();
            let before_zoom = camera.screen_to_world_f(mouse_pos);

            let old_zoom = camera.zoom;
            let target_zoom = (camera.zoom * (1.0 + wheel * 0.1)).clamp(0.1, 3.0);

            let lerp_factor = 0.5;
            camera.zoom = old_zoom + (target_zoom - old_zoom) * lerp_factor;

            if old_zoom != camera.zoom {
                let after_zoom = camera.screen_to_world_f(mouse_pos);
                camera.pos += before_zoom - after_zoom;
            }
        }
    }
}

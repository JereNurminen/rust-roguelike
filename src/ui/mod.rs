mod camera;
mod input_handler;
mod menu;
mod renderer;
mod types;

pub use camera::Camera;
pub use types::{MenuType, TILE_SIZE};

use crate::world::World;
use crate::world::WorldPosition;

pub struct UiState {
    renderer: renderer::Renderer,
    input_handler: input_handler::InputHandler,
    menu_renderer: menu::MenuRenderer,
    selected_tile: Option<WorldPosition>,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            renderer: renderer::Renderer::new(Camera::new()),
            input_handler: input_handler::InputHandler::new(),
            menu_renderer: menu::MenuRenderer::new(),
            selected_tile: None,
        }
    }

    pub fn update(&mut self) {
        // Store the result of handle_input in selected_tile
        if let Some(new_selection) = self.input_handler.handle_input(&mut self.renderer.camera) {
            self.selected_tile = Some(new_selection);
        }
    }

    pub fn render_game(&mut self, world: &World) {
        // Pass the current selected_tile to the renderer
        self.renderer.draw_world(world, self.selected_tile);
    }

    pub fn render_menu(&self, menu_type: &MenuType) {
        self.menu_renderer.render_menu(menu_type);
    }
}

use super::types::MenuType;
use macroquad::prelude::*;

pub struct MenuRenderer;

impl MenuRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render_menu(&self, menu_type: &MenuType) {
        match menu_type {
            MenuType::Inventory => self.render_inventory(),
            MenuType::Character => self.render_character(),
            MenuType::Pause => self.render_pause(),
        }
    }

    fn render_inventory(&self) {
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

    fn render_character(&self) {
        // Implementation for character menu
    }

    fn render_pause(&self) {
        // Implementation for pause menu
    }
}

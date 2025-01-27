use crate::game::Game;
use macroquad::window::Conf;

mod core;
mod game;
mod systems;
mod ui;
mod world;

fn window_conf() -> Conf {
    Conf {
        window_title: "Roguelike".to_owned(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        // You can customize more window settings here:
        // sample_count: 4,        // MSAA
        // high_dpi: true,        // Enable high DPI mode
        // window_resizable: true, // Allow window resizing
        // vsync: true,           // Enable vsync
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    game.run().await;
}

use eframe::{egui, App};

use crate::game::Game;

mod core;
mod game;
mod systems;
mod ui;
mod world;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size((800.0, 600.0))
            .with_title("roguelike"),
        ..Default::default()
    };
    eframe::run_native(
        "Roguelike",
        options,
        Box::new(|cc| {
            // Customize egui here with cc.egui_ctx if needed
            Ok(Box::new(Game::new(cc)))
        }),
    )
}

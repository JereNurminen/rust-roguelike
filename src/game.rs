use eframe::{egui, App};
use std::time::Instant;

use crate::ui::{MenuType, UiState};
use crate::world::{
    entity::{CoreAttributes, EntityId, Exhaustion, Status},
    Entity, World, WorldPosition,
};

pub enum GameState {
    PlayerTurn,
    ProcessingWorldTurns,
    AnimatingTurnTransition { pending_animations: Vec<usize> },
    InMenu(MenuType),
}

pub struct Game {
    world: World,
    player_entity_id: EntityId,
    ui: UiState,
    state: GameState,
    last_update: Instant,
}

impl Game {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Set up custom fonts if needed
        // cc.egui_ctx.set_fonts(...);
        //
        let player = Entity::new(
            0,
            crate::world::entity::EntityKind::Player,
            Some(WorldPosition { x: 0, y: 0 }),
            CoreAttributes::default(),
            Status {
                health: 10,
                stamina: 10,
                mana: 10,
                exhaustion: Exhaustion::Rested,
            },
        );

        Self {
            world: World::new(vec![player]),
            player_entity_id: 0,
            ui: UiState::new(),
            state: GameState::PlayerTurn,
            last_update: Instant::now(),
        }
    }

    fn update(&mut self, ctx: &egui::Context) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        self.world.update(dt);

        match &mut self.state {
            GameState::AnimatingTurnTransition { pending_animations } => {
                pending_animations.retain(|&entity_id| self.world.is_entity_animating(entity_id));
                if pending_animations.is_empty() {
                    self.state = GameState::PlayerTurn;
                }
            }
            _ => {}
        }
    }
}

impl App for Game {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui.render_game(ui, &self.world);
        });

        if let GameState::InMenu(menu_type) = &self.state {
            self.ui.render_menu(ctx, menu_type);
        }
    }
}

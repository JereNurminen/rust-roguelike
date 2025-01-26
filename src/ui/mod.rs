use crate::world::entity::EntityKind;
use crate::world::{World, WorldPosition};
use eframe::egui::{self, Color32, Pos2, Rect, Vec2};

pub struct UiState {
    camera: Camera,
    selected_tile: Option<WorldPosition>,
    open_menu: Option<MenuType>,
}

pub struct Camera {
    pos: WorldPosition,
    zoom: f32,
}

pub enum MenuType {
    Inventory,
    Character,
    Pause,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            camera: Camera {
                pos: WorldPosition::new(0, 0),
                zoom: 1.0,
            },
            selected_tile: None,
            open_menu: None,
        }
    }

    pub fn render_game(&mut self, ui: &mut egui::Ui, world: &World) {
        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

        // Handle input
        if response.dragged() {
            self.camera.pos.x -= (response.drag_delta().x / self.camera.zoom) as i32;
            self.camera.pos.y -= (response.drag_delta().y / self.camera.zoom) as i32;
        }
        if let Some(pos) = response.hover_pos() {
            if response.clicked() {
                self.selected_tile = Some(self.screen_to_world(pos, ui));
            }
        }

        let tile_size = 32.0 * self.camera.zoom;

        // Render entities
        for entity in world.entities().values().into_iter() {
            if let Some(position) = entity.position() {
                let screen_pos = self.world_to_screen(position, ui);

                match entity.kind() {
                    EntityKind::Floor { .. } => {
                        painter.rect_filled(
                            Rect::from_center_size(screen_pos, Vec2::splat(tile_size)),
                            0.0,
                            Color32::DARK_GRAY,
                        );
                    }
                    EntityKind::Wall { .. } => {
                        painter.rect_filled(
                            Rect::from_center_size(screen_pos, Vec2::splat(tile_size)),
                            0.0,
                            Color32::GRAY,
                        );
                    }
                    EntityKind::Npc { .. } => {
                        painter.circle_filled(screen_pos, tile_size / 2.0, Color32::RED);
                        painter.text(
                            screen_pos,
                            egui::Align2::CENTER_CENTER,
                            "X",
                            egui::FontId::proportional(tile_size as f32),
                            Color32::WHITE,
                        );
                    }
                    EntityKind::Player => {
                        painter.circle_filled(screen_pos, tile_size / 2.0, Color32::YELLOW);
                    }
                    _ => {}
                }
            }
        }

        // Render selection
        if let Some(pos) = self.selected_tile {
            let screen_pos = self.world_to_screen(pos, ui);
            painter.rect_stroke(
                Rect::from_center_size(screen_pos, Vec2::splat(tile_size)),
                0.0,
                (2.0, Color32::YELLOW),
            );
        }
    }

    pub fn render_menu(&self, ctx: &egui::Context, menu_type: &MenuType) {
        match menu_type {
            MenuType::Inventory => {
                egui::Window::new("Inventory").show(ctx, |ui| {
                    // Inventory UI here
                });
            }
            MenuType::Character => {
                egui::Window::new("Character").show(ctx, |ui| {
                    // Character UI here
                });
            }
            MenuType::Pause => {
                egui::Window::new("Pause").show(ctx, |ui| {
                    // Crafting UI here
                });
            }
        }
    }

    fn world_to_screen(&self, pos: WorldPosition, ui: &egui::Ui) -> Pos2 {
        let offset = Vec2::new(
            self.camera.pos.x as f32 * 32.0 * self.camera.zoom,
            self.camera.pos.y as f32 * 32.0 * self.camera.zoom,
        );
        Pos2::new(
            pos.x as f32 * 32.0 * self.camera.zoom - offset.x + ui.available_width() / 2.0,
            pos.y as f32 * 32.0 * self.camera.zoom - offset.y + ui.available_height() / 2.0,
        )
    }

    fn screen_to_world(&self, screen_pos: Pos2, ui: &egui::Ui) -> WorldPosition {
        let offset = Vec2::new(
            self.camera.pos.x as f32 * 32.0 * self.camera.zoom,
            self.camera.pos.y as f32 * 32.0 * self.camera.zoom,
        );
        WorldPosition::new(
            ((screen_pos.x + offset.x - ui.available_width() / 2.0) / (32.0 * self.camera.zoom))
                as i32,
            ((screen_pos.y + offset.y - ui.available_height() / 2.0) / (32.0 * self.camera.zoom))
                as i32,
        )
    }
}

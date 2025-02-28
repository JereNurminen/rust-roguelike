use engine::{
    application::{events::GameEvent, game_loop::GameState},
    core::types::Direction,
    create_initial_game_state,
    domain::{world::World, world_position::WorldPosition},
};
use serde::Serialize;
use specta::Type;
use specta_typescript::Typescript;
use std::sync::Mutex;
use tauri::State;
use tauri_specta::collect_commands;
use ts_rs::TS;

#[derive(Serialize, TS, Type)]
#[ts(export)]
struct ClientGameState {
    pub world: World,
}

impl From<World> for ClientGameState {
    fn from(state: World) -> ClientGameState {
        ClientGameState { world: state }
    }
}

type GameStateWrapper<'a> = State<'a, Mutex<GameState>>;

#[specta::specta]
#[tauri::command]
fn get_game_state(state: GameStateWrapper) -> Result<ClientGameState, String> {
    if let Ok(game_state) = state.lock() {
        Ok(ClientGameState::from(game_state.world.clone()))
    } else {
        Err("Error fetching game state from back end".to_string())
    }
}

#[specta::specta]
#[tauri::command]
fn move_player(state: GameStateWrapper, direction: Direction) -> Result<ClientGameState, String> {
    if let Ok(mut game_state) = state.lock() {
        let player_id = game_state.world.player_id;

        game_state.handle_event(GameEvent::MoveByDirection(player_id, direction));
        Ok(ClientGameState::from(game_state.world.clone()))
    } else {
        Err("Error applying player move event".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = Mutex::new(create_initial_game_state());

    tauri_specta::Builder::<tauri::Wry>::new()
        .commands(collect_commands![get_game_state, move_player])
        .export(
            Typescript::default().bigint(specta_typescript::BigIntExportBehavior::Number),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![get_game_state, move_player])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

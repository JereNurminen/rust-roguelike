use core::{
    create_initial_game_state, GameEvent, GameState,
    application::state_changes::{StateChange, StateChanges},
    core::types::Direction,
    domain::world_position::WorldPosition,
};
use std::sync::Mutex;
use tauri::State;
use serde_json::Value as JsonValue;

struct GameStateWrapper(Mutex<GameState>);

#[tauri::command]
fn get_game_state(state: State<GameStateWrapper>) -> Result<JsonValue, String> {
    let game_state = state.0.lock().map_err(|e| e.to_string())?;
    
    let entities = game_state.world.entities.values().map(|entity| {
        serde_json::json!({
            "id": entity.id,
            "kind": format!("{:?}", entity.kind()),
            "pos": entity.pos().map(|p| {
                serde_json::json!({
                    "x": p.x,
                    "y": p.y
                })
            })
        })
    }).collect::<Vec<_>>();
    
    Ok(serde_json::json!({ "entities": entities }))
}

#[tauri::command]
fn get_entities_at_position(
    x: i32,
    y: i32,
    state: State<GameStateWrapper>
) -> Result<JsonValue, String> {
    let game_state = state.0.lock().map_err(|e| e.to_string())?;
    let pos = WorldPosition::new(x, y);
    
    let entities = game_state.world.get_entities_by_pos(&pos)
        .into_iter()
        .map(|entity| {
            serde_json::json!({
                "id": entity.id,
                "kind": format!("{:?}", entity.kind()),
                "pos": entity.pos().map(|p| {
                    serde_json::json!({
                        "x": p.x,
                        "y": p.y
                    })
                })
            })
        })
        .collect::<Vec<_>>();
    
    Ok(serde_json::json!(entities))
}

#[tauri::command]
fn move_player(
    direction: String,
    state: State<GameStateWrapper>
) -> Result<StateChanges, String> {
    let mut game_state = state.0.lock().map_err(|e| e.to_string())?;
    
    let direction = match direction.as_str() {
        "North" => Direction::North,
        "South" => Direction::South,
        "East" => Direction::East,
        "West" => Direction::West,
        _ => return Err("Invalid direction".to_string()),
    };
    
    if let Some(player_id) = game_state.get_current_entity() {
        let changes = game_state.handle_event(GameEvent::MoveByDirection(player_id, direction));
        if !changes.is_empty() {
            let end_turn_changes = game_state.handle_event(GameEvent::EndTurn(player_id));
            Ok([changes, end_turn_changes].concat())
        } else {
            Ok(changes)
        }
    } else {
        Err("No current entity".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let game_state = GameStateWrapper(Mutex::new(create_initial_game_state()));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(game_state)
        .invoke_handler(tauri::generate_handler![
            get_game_state,
            get_entities_at_position,
            move_player
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

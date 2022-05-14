#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod entity;
mod combat;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_character, create_enemy, resolve_combat])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn create_character(name: String, stamina: i32, strength: i32) -> entity::Entity {
    let character = entity::Entity {
        name,
        gold: 0,
        stamina,
        strength,
    };
    println!("Name: {} Stamina: {} Strength: {}", character.name, character.stamina, character.strength);

    character.into()
}

#[tauri::command]
fn create_enemy() -> entity::Entity {
    let enemy = entity::Entity {
        name: String::from("Spider"),
        gold: 10,
        stamina: 8,
        strength: 2,
    };

    enemy.into()
}

#[tauri::command]
fn resolve_combat(character: entity::Entity, enemy: entity::Entity) -> Vec<combat::CombatStep> {
    combat::resolve_combat(&character, &enemy)
}

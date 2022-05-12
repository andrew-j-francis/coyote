#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod entity;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_character])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn create_character(name: String, stamina: i32, strength: i32) -> entity::Character {
    let character = entity::Character {
        name,
        gold: 0,
        stamina,
        strength,
    };
    println!("Name: {} Stamina: {} Strength: {}", character.name, character.stamina, character.strength);

    character.into()
}

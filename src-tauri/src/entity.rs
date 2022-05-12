use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub gold: i32,
    pub stamina: i32,
    pub strength: i32,
}
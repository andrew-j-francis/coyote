use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub gold: i32,
    pub stamina: i32,
    pub strength: i32,
}

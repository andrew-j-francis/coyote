use crate::entity::Entity;
use serde::{Serialize, Deserialize};

pub fn resolve_combat(character: &Entity, enemy: &Entity) -> Vec<CombatStep> {
    let mut combat_steps = Vec::new();

    let character_attack_power = character.strength * 10;
    let mut character_health = character.stamina * 10;

    let enemy_attack_power = enemy.strength * 10;
    let mut enemy_health = enemy.stamina * 10;

    let mut step_number = 1;

    loop {
        enemy_health -= character_attack_power;

        if enemy_health <= 0 {
            combat_steps.push(CombatStep {
                step_number,
                character_damage: character_attack_power,
                enemy_damage: 0,
                enemy_is_dead: true,
            });

            return combat_steps;
        }

        character_health -= enemy_attack_power;

        combat_steps.push(CombatStep {
            step_number,
            character_damage: character_attack_power,
            enemy_damage: enemy_attack_power,
            enemy_is_dead: false,
        });

        if character_health <= 0 {
            return combat_steps;
        }

        step_number += 1;
    }
}

#[derive(Serialize, Deserialize)]
pub struct CombatStep {
    pub step_number: u32,
    pub character_damage: i32,
    pub enemy_damage: i32,
    pub enemy_is_dead: bool,
}

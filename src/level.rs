use super::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Level {
  pub background: Vec<Entity>,
  pub indestructible: Vec<Entity>,
  pub destructible: Vec<Entity>,
  pub enemies: Vec<Entity>,
  pub main_character: Vec<Entity>,
  pub effects: Vec<Entity>,
  pub foreground: Vec<Entity>,
}

impl Level {
  pub fn serialize(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }
  pub fn deserialize(serialized: String) -> Self {
    serde_json::from_str(&serialized).unwrap()
  }
}

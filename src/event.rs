use super::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum EventType {
  Kill,
  Teleport(f32, f32),
}

#[derive(Serialize, Deserialize)]
pub struct Event {
  pub entity: Entity,
  pub event_type: EventType,
  #[serde(default = "default_receiving_entity_ids")]
  pub receiving_entity_ids: Vec<String>,
}

fn default_receiving_entity_ids() -> Vec<String> {
  vec![]
}

impl Event {
  pub fn is_triggering(&self, entity: &Entity) -> bool {
    entity.is_inside_entity(&self.entity)
      && (self.receiving_entity_ids.len() == 0
        || match &entity.id {
          Some(id) => self.receiving_entity_ids.iter().any(|receiving_id| receiving_id == id),
          None => false,
        })
  }
}

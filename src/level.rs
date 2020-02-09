use super::camera::Camera;
use super::entity::Entity;
use sdl2::render::{Texture, WindowCanvas};
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
  pub fn next_state(&mut self, entities: &Vec<Entity>) {
    Level::next_container_state(&mut self.main_character, &entities);
    Level::next_container_state(&mut self.effects, &entities);
    Level::next_container_state(&mut self.destructible, &entities);
    Level::next_container_state(&mut self.indestructible, &entities);
    Level::next_container_state(&mut self.enemies, &entities);
  }
  fn next_container_state(container: &mut Vec<Entity>, entities: &Vec<Entity>) {
    container.drain_filter(|entity| {
      entity.next_state(&entities);
      Some("dying".to_string()) == entity.id
    });
  }

  pub fn draw(&self, camera: &mut Camera, canvas: &mut WindowCanvas, texture: &Texture) {
    Level::draw_container(&self.background, camera, canvas, texture);
    Level::draw_container(&self.indestructible, camera, canvas, texture);
    Level::draw_container(&self.destructible, camera, canvas, texture);
    Level::draw_container(&self.enemies, camera, canvas, texture);
    Level::draw_container(&self.main_character, camera, canvas, texture);
    Level::draw_container(&self.effects, camera, canvas, texture);
    Level::draw_container(&self.foreground, camera, canvas, texture);
  }
  fn draw_container(
    container: &Vec<Entity>,
    camera: &mut Camera,
    canvas: &mut WindowCanvas,
    texture: &Texture,
  ) {
    for entity in container {
      camera.draw_relatively(canvas, &entity, &texture);
    }
  }
  pub fn serialize(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }
  pub fn deserialize(serialized: String) -> Self {
    serde_json::from_str(&serialized).unwrap()
  }
}

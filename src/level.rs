use super::camera::Camera;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
  pub sprite_sheet_rect: Option<(i32, i32, u32, u32)>,
  pub aim_direction: Option<f32>,
  #[serde(default = "default_bounciness")]
  pub bounciness: f32,
  #[serde(default = "default_slippiness")]
  pub slippiness: f32,
  pub dimensions: (u32, u32),
  pub position: (f32, f32),
  #[serde(default = "default_velocity")]
  pub velocity: (f32, f32),
  #[serde(default = "default_acceleration")]
  pub acceleration: (f32, f32),
  #[serde(default = "default_parallax")]
  pub parallax: (f32, f32),
}

fn default_bounciness() -> f32 {
  0.4
}
fn default_slippiness() -> f32 {
  0.8
}
fn default_velocity() -> (f32, f32) {
  (0.0, 0.0)
}
fn default_acceleration() -> (f32, f32) {
  (0.0, 0.0)
}
fn default_parallax() -> (f32, f32) {
  (1.0, 1.0)
}

impl Entity {
  pub fn new(x: f32, y: f32, width: u32, height: u32) -> Self {
    Self {
      sprite_sheet_rect: None,
      aim_direction: None,
      bounciness: 0.4,
      slippiness: 0.8,
      dimensions: (width, height),
      position: (x, y),
      velocity: (0.0, 0.0),
      acceleration: (0.0, 1.0),
      parallax: (1.0, 1.0),
    }
  }
  pub fn parallax_x(mut self, parallax_x: f32) -> Self {
    self.parallax.0 = parallax_x;
    self
  }
  pub fn parallax_y(mut self, parallax_y: f32) -> Self {
    self.parallax.1 = parallax_y;
    self
  }
  pub fn velocity_x(mut self, velocity_x: f32) -> Self {
    self.velocity.0 = velocity_x;
    self
  }
  pub fn velocity_y(mut self, velocity_y: f32) -> Self {
    self.velocity.1 = velocity_y;
    self
  }
  pub fn bounciness(mut self, bounciness: f32) -> Self {
    self.bounciness = bounciness;
    self
  }
  pub fn is_touching_ground(&self, interactive_entities: &Vec<Self>) -> bool {
    match self.find_ground_entity(interactive_entities) {
      Some(_) => true,
      None => false,
    }
  }
  pub fn find_ground_entity<'a>(&self, interactive_entities: &'a Vec<Self>) -> Option<&'a Entity> {
    let lower_end = self.position.1 as i32 + self.dimensions.1 as i32;
    interactive_entities.iter().find(|entity| {
      entity.position.1 as i32 == lower_end
        && ((entity.position.0 < self.position.0
          && entity.position.0 + entity.dimensions.0 as f32 > self.position.0)
          || (entity.position.0 < self.position.0 + self.dimensions.0 as f32
            && entity.position.0 + entity.dimensions.0 as f32
              > self.position.0 + self.dimensions.0 as f32))
    })
  }
  pub fn to_canvas_coordinates(&self, camera: &Camera, offset: (u32, u32)) -> (f32, f32, u32, u32) {
    (
      self.position.0 * camera.scale.0 - camera.position.0 * (self.parallax.0 * camera.scale.0)
        + offset.0 as f32,
      self.position.1 * camera.scale.1 - camera.position.1 * (self.parallax.1 * camera.scale.1)
        + offset.1 as f32,
      (self.dimensions.0 as f32 * camera.scale.0) as u32,
      (self.dimensions.1 as f32 * camera.scale.1) as u32,
    )
  }

  pub fn from_canvas_coordinates(
    (x, y, width, height, parallax_x, parallax_y): (f32, f32, u32, u32, f32, f32),
    camera: &Camera,
    offset: (u32, u32),
  ) -> Self {
    Entity::new(
      (x + camera.position.0 * parallax_x * camera.scale.0 - offset.0 as f32) / camera.scale.0,
      (y + camera.position.1 * parallax_y * camera.scale.1 - offset.1 as f32) / camera.scale.1,
      (width as f32 / camera.scale.0) as u32,
      (height as f32 / camera.scale.1) as u32,
    )
    .parallax_x(parallax_x)
    .parallax_y(parallax_y)
  }
  pub fn next_state(&mut self, interactive_entities: &Vec<Self>) {
    if let Some(ground_entity) = self.find_ground_entity(&interactive_entities) {
      self.velocity.0 *= self.slippiness.max(ground_entity.slippiness);
    }

    self.velocity.0 += self.acceleration.0;
    self.velocity.1 += self.acceleration.1;

    let position_before = self.position;

    self.position.0 += self.velocity.0;
    self.position.1 += self.velocity.1;

    let collided_entities = interactive_entities
      .iter()
      .filter(|&entity| entity.is_inside_entity(self))
      .collect::<Vec<_>>();

    if let Some(right_to_self) = collided_entities
      .iter()
      .find(|entity| entity.position.0 >= position_before.0 + self.dimensions.0 as f32)
    {
      self.position.0 = right_to_self.position.0 - self.dimensions.0 as f32;
      self.velocity.0 *= -1.0 * self.bounciness * right_to_self.bounciness;
    } else if let Some(left_to_self) = collided_entities
      .iter()
      .find(|entity| entity.position.0 + entity.dimensions.0 as f32 <= position_before.0)
    {
      self.position.0 = left_to_self.position.0 + left_to_self.dimensions.0 as f32;
      self.velocity.0 *= -1.0 * self.bounciness * left_to_self.bounciness;
    }
    if let Some(bottom_to_self) = collided_entities
      .iter()
      .find(|entity| entity.position.1 >= position_before.1 + self.dimensions.1 as f32)
    {
      self.position.1 = bottom_to_self.position.1 - self.dimensions.1 as f32;
      self.velocity.1 *= -1.0 * self.bounciness * bottom_to_self.bounciness;
    } else if let Some(top_to_self) = collided_entities
      .iter()
      .find(|entity| entity.position.1 + entity.dimensions.1 as f32 <= position_before.1)
    {
      self.position.1 = top_to_self.position.1 + top_to_self.dimensions.1 as f32;
      self.velocity.1 *= -1.0 * self.bounciness * top_to_self.bounciness;
    }
  }
  pub fn is_inside_entity(&self, entity: &Entity) -> bool {
    (self.position.0 + self.dimensions.0 as f32 >= entity.position.0
      && self.position.1 + self.dimensions.1 as f32 >= entity.position.1
      && self.position.0 <= entity.position.0 + entity.dimensions.0 as f32
      && self.position.1 <= entity.position.1 + entity.dimensions.1 as f32)
  }
}

#[derive(Serialize, Deserialize)]
pub enum Event {
  Kill,
}

#[derive(Serialize, Deserialize)]
pub struct EventEntity {
  pub entity: Entity,
  pub event: Event,
}

#[derive(Serialize, Deserialize)]
pub struct Level {
  pub background: Vec<Entity>,
  pub indestructible: Vec<Entity>,
  pub destructible: Vec<Entity>,
  pub enemies: Vec<Entity>,
  pub main_character: Vec<Entity>,
  pub effects: Vec<Entity>,
  pub foreground: Vec<Entity>,
  pub event_entities: Vec<EventEntity>,
}

impl Level {
  pub fn serialize(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }
  pub fn deserialize(serialized: String) -> Self {
    serde_json::from_str(&serialized).unwrap()
  }
}

#[test]
fn window_entity_coordinates_vs_actual_coordinates_entity_at_center() {
  let mut camera = Camera::new((900, 600));
  camera.position.0 = -450.0;
  camera.position.1 = -300.0;

  let entity = Entity::new(0.0, 0.0, 10, 10);
  let (x, y, width, height) = entity.to_canvas_coordinates(&camera, (450, 300));
  let entity_like_at_the_beginning =
    Entity::from_canvas_coordinates((x, y, width, height, 1.0, 1.0), &camera, (450, 300));

  assert_eq!(x, 900.0);
  assert_eq!(y, 600.0);
  assert_eq!(width, 10);
  assert_eq!(height, 10);

  assert_eq!(entity.position.0, entity_like_at_the_beginning.position.0);
  assert_eq!(entity.position.1, entity_like_at_the_beginning.position.1);
  assert_eq!(entity.dimensions.0, entity_like_at_the_beginning.dimensions.0);
  assert_eq!(entity.dimensions.1, entity_like_at_the_beginning.dimensions.1);
}

#[test]
fn window_entity_coordinates_vs_actual_coordinates_entity_at_start() {
  let camera = Camera::new((900, 600));

  let entity = Entity::new(0.0, 0.0, 10, 10);
  let (x, y, width, height) = entity.to_canvas_coordinates(&camera, (450, 300));
  let entity_like_at_the_beginning =
    Entity::from_canvas_coordinates((x, y, width, height, 1.0, 1.0), &camera, (450, 300));

  assert_eq!(x, 450.0);
  assert_eq!(y, 300.0);
  assert_eq!(width, 10);
  assert_eq!(height, 10);

  assert_eq!(entity.position.0, entity_like_at_the_beginning.position.0);
  assert_eq!(entity.position.1, entity_like_at_the_beginning.position.1);
  assert_eq!(entity.dimensions.0, entity_like_at_the_beginning.dimensions.0);
  assert_eq!(entity.dimensions.1, entity_like_at_the_beginning.dimensions.1);
}

#[test]
fn window_entity_coordinates_vs_actual_coordinates() {
  let mut camera = Camera::new((900, 600));
  camera.position.0 = 450.0;

  let entity = Entity::new(600.0, 0.0, 10, 10);
  let (x, y, width, height) = entity.to_canvas_coordinates(&camera, (450, 300));
  let entity_like_at_the_beginning =
    Entity::from_canvas_coordinates((x, y, width, height, 1.0, 1.0), &camera, (450, 300));

  assert_eq!(x, 600.0);
  assert_eq!(y, 300.0);
  assert_eq!(width, 10);
  assert_eq!(height, 10);

  assert_eq!(entity.position.0, entity_like_at_the_beginning.position.0);
  assert_eq!(entity.position.1, entity_like_at_the_beginning.position.1);
  assert_eq!(entity.dimensions.0, entity_like_at_the_beginning.dimensions.0);
  assert_eq!(entity.dimensions.1, entity_like_at_the_beginning.dimensions.1);
}

#[test]
fn window_entity_coordinates_vs_actual_coordinates_plus_scale() {
  let mut camera = Camera::new((900, 600));
  camera.zoom(2.0);
  camera.position.0 = 450.0;

  let entity = Entity::new(600.0, 0.0, 10, 10);
  let (x, y, width, height) = entity.to_canvas_coordinates(&camera, (450, 300));
  let entity_like_at_the_beginning =
    Entity::from_canvas_coordinates((x, y, width, height, 1.0, 1.0), &camera, (450, 300));

  assert_eq!(x, 750.0);
  assert_eq!(y, 300.0);
  assert_eq!(width, 20);
  assert_eq!(height, 20);

  assert_eq!(entity.position.0, entity_like_at_the_beginning.position.0);
  assert_eq!(entity.position.1, entity_like_at_the_beginning.position.1);
  assert_eq!(entity.dimensions.0, entity_like_at_the_beginning.dimensions.0);
  assert_eq!(entity.dimensions.1, entity_like_at_the_beginning.dimensions.1);
}

use super::camera::Camera;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
  Kill,
  Teleport(f32, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
  pub event_type: EventType,
  #[serde(default = "default_receiving_entity_ids")]
  pub receiving_entity_ids: Vec<String>,
}

fn default_receiving_entity_ids() -> Vec<String> {
  vec![]
}

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
  pub id: Option<String>,
  pub event: Option<Event>,
  pub health: Option<i32>,
  pub damage_factor: Option<f32>,
  #[serde(default = "default_step_height")]
  pub step_height: f32,
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
fn default_step_height() -> f32 {
  0.0
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
      id: None,
      step_height: default_step_height(),
      sprite_sheet_rect: None,
      aim_direction: None,
      event: None,
      health: None,
      damage_factor: None,
      bounciness: default_bounciness(),
      slippiness: default_slippiness(),
      dimensions: (width, height),
      position: (x, y),
      velocity: default_velocity(),
      acceleration: (0.0, 1.0),
      parallax: default_parallax(),
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
  pub fn id(mut self, id: String) -> Self {
    self.id = Some(id);
    self
  }
  pub fn step_height(mut self, step_height: f32) -> Self {
    self.step_height = step_height;
    self
  }
  pub fn damage_factor(mut self, damage_factor: Option<f32>) -> Self {
    self.damage_factor = damage_factor;
    self
  }
  fn is_triggering(&self, entity: &Entity) -> bool {
    if let Some(event) = &entity.event {
      event.receiving_entity_ids.len() == 0
        || match &self.id {
          Some(id) => event.receiving_entity_ids.iter().any(|receiving_id| receiving_id == id),
          None => false,
        }
    } else {
      false
    }
  }
  fn run_event(&mut self, event: &Event) {
    match event.event_type {
      EventType::Teleport(x, y) => {
        self.position.0 = x;
        self.position.1 = y;
      }
      EventType::Kill => {
        self.id = Some("dying".to_string());
      }
    }
  }
  pub fn is_touching_ground(&self, interactive_entities: &Vec<Self>) -> bool {
    match self.find_ground_entity(interactive_entities) {
      Some(_) => true,
      None => false,
    }
  }
  fn find_ground_entity<'a>(&self, interactive_entities: &'a Vec<Self>) -> Option<&'a Entity> {
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

    let mut is_on_ground = false;

    if let Some(bottom_to_self) = collided_entities
      .iter()
      .find(|entity| entity.position.1 >= position_before.1 + self.dimensions.1 as f32)
    {
      if self.is_triggering(bottom_to_self) {
        self.run_event(bottom_to_self.event.as_ref().unwrap());
      } else {
        self.position.1 = bottom_to_self.position.1 - self.dimensions.1 as f32;
        self.velocity.1 *= -1.0 * self.bounciness * bottom_to_self.bounciness;

        if let Some(health) = bottom_to_self.health {
          if let Some(damage_factor) = self.damage_factor {
            self.id = Some("dying".to_string());
          }
        }
      }
      is_on_ground = true;
    } else if let Some(top_to_self) = collided_entities
      .iter()
      .find(|entity| entity.position.1 + entity.dimensions.1 as f32 <= position_before.1)
    {
      if self.is_triggering(top_to_self) {
        self.run_event(top_to_self.event.as_ref().unwrap());
      } else {
        self.position.1 = top_to_self.position.1 + top_to_self.dimensions.1 as f32;
        self.velocity.1 *= -1.0 * self.bounciness * top_to_self.bounciness;
        if let Some(health) = top_to_self.health {
          if let Some(damage_factor) = self.damage_factor {
            self.id = Some("dying".to_string());
          }
        }
      }
    }
    if let Some(right_to_self) = collided_entities
      .iter()
      .find(|entity| entity.position.0 >= position_before.0 + self.dimensions.0 as f32)
    {
      if self.is_triggering(right_to_self) {
        self.run_event(right_to_self.event.as_ref().unwrap());
      } else {
        if is_on_ground
          && (self.position.1 + self.dimensions.1 as f32) - right_to_self.position.1
            <= self.step_height
        {
          self.position.1 = right_to_self.position.1 - self.dimensions.1 as f32;
        } else {
          self.position.0 = right_to_self.position.0 - self.dimensions.0 as f32;
          self.velocity.0 *= -1.0 * self.bounciness * right_to_self.bounciness;
          if let Some(health) = right_to_self.health {
            if let Some(damage_factor) = self.damage_factor {
              self.id = Some("dying".to_string());
            }
          }
        }
      }
    } else if let Some(left_to_self) = collided_entities
      .iter()
      .find(|entity| entity.position.0 + entity.dimensions.0 as f32 <= position_before.0)
    {
      if self.is_triggering(left_to_self) {
        self.run_event(left_to_self.event.as_ref().unwrap());
      } else {
        if is_on_ground
          && (self.position.1 + self.dimensions.1 as f32) - left_to_self.position.1
            <= self.step_height
        {
          self.position.1 = left_to_self.position.1 - self.dimensions.1 as f32;
        } else {
          self.position.0 = left_to_self.position.0 + left_to_self.dimensions.0 as f32;
          self.velocity.0 *= -1.0 * self.bounciness * left_to_self.bounciness;
          if let Some(health) = left_to_self.health {
            if let Some(damage_factor) = self.damage_factor {
              self.id = Some("dying".to_string());
            }
          }
        }
      }
    }
  }
  pub fn is_inside_entity(&self, entity: &Entity) -> bool {
    self.position.0 + self.dimensions.0 as f32 >= entity.position.0
      && self.position.1 + self.dimensions.1 as f32 >= entity.position.1
      && self.position.0 <= entity.position.0 + entity.dimensions.0 as f32
      && self.position.1 <= entity.position.1 + entity.dimensions.1 as f32
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

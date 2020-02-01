use super::camera::Camera;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
  pub sprite_sheet_rect: Option<(i32, i32, u32, u32)>,
  pub bounciness: f32,
  pub width: u32,
  pub height: u32,
  pub x: f32,
  pub y: f32,
  pub velocity_x: f32,
  pub velocity_y: f32,
  pub acceleration_x: f32,
  pub acceleration_y: f32,
  pub parallax_x: f32,
  pub parallax_y: f32,
}
impl Entity {
  pub fn new(x: f32, y: f32, width: u32, height: u32) -> Self {
    Self {
      sprite_sheet_rect: None,
      bounciness: 0.4,
      width,
      height,
      x,
      y,
      velocity_x: 0.0,
      velocity_y: 0.0,
      acceleration_x: 0.0,
      acceleration_y: 1.0,
      parallax_x: 1.0,
      parallax_y: 1.0,
    }
  }
  pub fn parallax_x(mut self, parallax_x: f32) -> Self {
    self.parallax_x = parallax_x;
    self
  }
  pub fn parallax_y(mut self, parallax_y: f32) -> Self {
    self.parallax_y = parallax_y;
    self
  }
  pub fn velocity_x(mut self, velocity_x: f32) -> Self {
    self.velocity_x = velocity_x;
    self
  }
  pub fn velocity_y(mut self, velocity_y: f32) -> Self {
    self.velocity_y = velocity_y;
    self
  }
  pub fn acceleration_x(mut self, acceleration_x: f32) -> Self {
    self.acceleration_x = acceleration_x;
    self
  }
  pub fn acceleration_y(mut self, acceleration_y: f32) -> Self {
    self.acceleration_y = acceleration_y;
    self
  }
  pub fn bounciness(mut self, bounciness: f32) -> Self {
    self.bounciness = bounciness;
    self
  }
  pub fn is_touching_ground(&mut self, interactive_entities: Vec<&Self>) -> bool {
    let lower_end = self.y as i32 + self.height as i32;
    interactive_entities.iter().any(|entity| {
      entity.y as i32 == lower_end
        && ((entity.x < self.x && entity.x + entity.width as f32 > self.x)
          || (entity.x < self.x + self.width as f32
            && entity.x + entity.width as f32 > self.x + self.width as f32))
    })
  }
  pub fn to_canvas_coordinates(&self, camera: &Camera, offset: (u32, u32)) -> (f32, f32, u32, u32) {
    (
      self.x * camera.get_scale_x() - camera.get_x() * (self.parallax_x * camera.get_scale_x())
        + offset.0 as f32,
      self.y * camera.get_scale_y() - camera.get_y() * (self.parallax_y * camera.get_scale_y())
        + offset.1 as f32,
      (self.width as f32 * camera.get_scale_x()) as u32,
      (self.height as f32 * camera.get_scale_y()) as u32,
    )
  }

  pub fn from_canvas_coordinates(
    (x, y, width, height, parallax_x, parallax_y): (f32, f32, u32, u32, f32, f32),
    camera: &Camera,
    offset: (u32, u32),
  ) -> Self {
    Entity::new(
      (x + camera.get_x() * parallax_x * camera.get_scale_x() - offset.0 as f32)
        / camera.get_scale_x(),
      (y + camera.get_y() * parallax_y * camera.get_scale_y() - offset.1 as f32)
        / camera.get_scale_y(),
      (width as f32 / camera.get_scale_x()) as u32,
      (height as f32 / camera.get_scale_y()) as u32,
    )
    .parallax_x(parallax_x)
    .parallax_y(parallax_y)
  }
  pub fn next_state(&mut self, mut interactive_entities: Vec<&Self>) {
    self.velocity_x += self.acceleration_x;
    self.velocity_y += self.acceleration_y;

    let (x_before, y_before) = (self.x, self.y);

    self.x += self.velocity_x;
    self.y += self.velocity_y;

    interactive_entities
      .retain(|entity| entity.is_inside_bounds(self.x, self.y, self.width, self.height));

    if interactive_entities.len() > 0 {
      if let Some(right_to_self) =
        interactive_entities.iter().find(|entity| entity.x >= x_before + self.width as f32)
      {
        self.x = right_to_self.x - self.width as f32;
        self.velocity_x *= -1.0 * self.bounciness * right_to_self.bounciness;
      } else if let Some(left_to_self) =
        interactive_entities.iter().find(|entity| entity.x + entity.width as f32 <= x_before)
      {
        self.x = left_to_self.x + left_to_self.width as f32;
        self.velocity_x *= -1.0 * self.bounciness * left_to_self.bounciness;
      }
      if let Some(bottom_to_self) =
        interactive_entities.iter().find(|entity| entity.y >= y_before + self.height as f32)
      {
        self.y = bottom_to_self.y - self.height as f32;
        self.velocity_y *= -1.0 * self.bounciness * bottom_to_self.bounciness;
      } else if let Some(top_to_self) =
        interactive_entities.iter().find(|entity| entity.y + entity.height as f32 <= y_before)
      {
        self.y = top_to_self.y + top_to_self.height as f32;
        self.velocity_y *= -1.0 * self.bounciness * top_to_self.bounciness;
      }
    }
  }

  pub fn is_inside_bounds(&self, x: f32, y: f32, width: u32, height: u32) -> bool {
    (self.x + self.width as f32 >= x
      && self.y + self.height as f32 >= y
      && self.x <= x + width as f32
      && self.y <= y + height as f32)
  }
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

  assert_eq!(entity.x, entity_like_at_the_beginning.x);
  assert_eq!(entity.y, entity_like_at_the_beginning.y);
  assert_eq!(entity.width, entity_like_at_the_beginning.width);
  assert_eq!(entity.height, entity_like_at_the_beginning.height);
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

  assert_eq!(entity.x, entity_like_at_the_beginning.x);
  assert_eq!(entity.y, entity_like_at_the_beginning.y);
  assert_eq!(entity.width, entity_like_at_the_beginning.width);
  assert_eq!(entity.height, entity_like_at_the_beginning.height);
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

  assert_eq!(entity.x, entity_like_at_the_beginning.x);
  assert_eq!(entity.y, entity_like_at_the_beginning.y);
  assert_eq!(entity.width, entity_like_at_the_beginning.width);
  assert_eq!(entity.height, entity_like_at_the_beginning.height);
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

  assert_eq!(entity.x, entity_like_at_the_beginning.x);
  assert_eq!(entity.y, entity_like_at_the_beginning.y);
  assert_eq!(entity.width, entity_like_at_the_beginning.width);
  assert_eq!(entity.height, entity_like_at_the_beginning.height);
}

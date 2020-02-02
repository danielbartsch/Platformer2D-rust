use super::camera::Camera;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
  pub sprite_sheet_rect: Option<(i32, i32, u32, u32)>,
  pub aim_direction: Option<f32>,
  pub bounciness: f32,
  pub dimensions: (u32, u32),
  pub position: (f32, f32),
  pub velocity: (f32, f32),
  pub acceleration: (f32, f32),
  pub parallax: (f32, f32),
}
impl Entity {
  pub fn new(x: f32, y: f32, width: u32, height: u32) -> Self {
    Self {
      sprite_sheet_rect: None,
      aim_direction: None,
      bounciness: 0.4,
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
  pub fn acceleration_x(mut self, acceleration_x: f32) -> Self {
    self.acceleration.0 = acceleration_x;
    self
  }
  pub fn acceleration_y(mut self, acceleration_y: f32) -> Self {
    self.acceleration.1 = acceleration_y;
    self
  }
  pub fn bounciness(mut self, bounciness: f32) -> Self {
    self.bounciness = bounciness;
    self
  }
  pub fn is_touching_ground(&self, interactive_entities: &Vec<Self>) -> bool {
    let lower_end = self.position.1 as i32 + self.dimensions.1 as i32;
    interactive_entities.iter().any(|entity| {
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
      self.position.0 * camera.get_scale_x()
        - camera.get_x() * (self.parallax.0 * camera.get_scale_x())
        + offset.0 as f32,
      self.position.1 * camera.get_scale_y()
        - camera.get_y() * (self.parallax.1 * camera.get_scale_y())
        + offset.1 as f32,
      (self.dimensions.0 as f32 * camera.get_scale_x()) as u32,
      (self.dimensions.1 as f32 * camera.get_scale_y()) as u32,
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
  pub fn next_state(&mut self, interactive_entities: &Vec<Self>) {
    self.velocity.0 += self.acceleration.0;
    self.velocity.1 += self.acceleration.1;

    let (x_before, y_before, width, height) =
      (self.position.0, self.position.1, self.dimensions.0, self.dimensions.1);

    self.position.0 += self.velocity.0;
    self.position.1 += self.velocity.1;

    let (x_after, y_after) = (self.position.0, self.position.1);

    let collided_entities = interactive_entities
      .iter()
      .filter(|&entity| entity.is_inside_bounds(x_after, y_after, width, height))
      .collect::<Vec<_>>();

    if let Some(right_to_self) =
      collided_entities.iter().find(|entity| entity.position.0 >= x_before + width as f32)
    {
      self.position.0 = right_to_self.position.0 - width as f32;
      self.velocity.0 *= -1.0 * self.bounciness * right_to_self.bounciness;
    } else if let Some(left_to_self) = collided_entities
      .iter()
      .find(|entity| entity.position.0 + entity.dimensions.0 as f32 <= x_before)
    {
      self.position.0 = left_to_self.position.0 + left_to_self.dimensions.0 as f32;
      self.velocity.0 *= -1.0 * self.bounciness * left_to_self.bounciness;
    }
    if let Some(bottom_to_self) =
      collided_entities.iter().find(|entity| entity.position.1 >= y_before + height as f32)
    {
      self.position.1 = bottom_to_self.position.1 - height as f32;
      self.velocity.1 *= -1.0 * self.bounciness * bottom_to_self.bounciness;
    } else if let Some(top_to_self) = collided_entities
      .iter()
      .find(|entity| entity.position.1 + entity.dimensions.1 as f32 <= y_before)
    {
      self.position.1 = top_to_self.position.1 + top_to_self.dimensions.1 as f32;
      self.velocity.1 *= -1.0 * self.bounciness * top_to_self.bounciness;
    }
  }
  pub fn is_inside_bounds(&self, x: f32, y: f32, width: u32, height: u32) -> bool {
    (self.position.0 + self.dimensions.0 as f32 >= x
      && self.position.1 + self.dimensions.1 as f32 >= y
      && self.position.0 <= x + width as f32
      && self.position.1 <= y + height as f32)
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

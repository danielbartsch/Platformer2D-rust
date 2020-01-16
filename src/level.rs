use super::camera::Camera;
use sdl2::rect::Rect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityVariant {
    Block,
    Platform,
    MainCharacter,
    Pillar,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub variant: EntityVariant,
    pub bouncyness: f32,
    pub width: u16,
    pub height: u16,
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
    pub fn new(x: f32, y: f32, width: u16, height: u16) -> Self {
        Self {
            variant: EntityVariant::Platform,
            bouncyness: 0.4,
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
    pub fn variant(mut self, variant: EntityVariant) -> Self {
        self.variant = variant;
        self
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
    pub fn bouncyness(mut self, bouncyness: f32) -> Self {
        self.bouncyness = bouncyness;
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
    pub fn to_canvas_coordinates(&self, camera: &Camera) -> (f32, f32, u16, u16) {
        (
            self.x * camera.get_scale_x()
                - camera.get_x() * (self.parallax_x * camera.get_scale_x()),
            self.y * camera.get_scale_y()
                - camera.get_y() * (self.parallax_y * camera.get_scale_y()),
            (self.width as f32 * camera.get_scale_x()) as u16,
            (self.height as f32 * camera.get_scale_y()) as u16,
        )
    }
    pub fn from_canvas_coordinates(
        (x, y, width, height, parallax_x, parallax_y): (f32, f32, u16, u16, f32, f32),
        camera: &Camera,
    ) -> Self {
        Entity::new(
            (x + camera.get_x() * parallax_x * camera.get_scale_x()) / camera.get_scale_x(),
            (y + camera.get_y() * parallax_y * camera.get_scale_y()) / camera.get_scale_y(),
            (width as f32 / camera.get_scale_x()) as u16,
            (height as f32 / camera.get_scale_y()) as u16,
        )
    }
    pub fn next_state(&mut self, mut interactive_entities: Vec<&Self>) {
        let intended_velocity = (
            self.velocity_x + self.acceleration_x,
            self.velocity_y + self.acceleration_y,
        );
        let intended_position = (
            (self.x + intended_velocity.0 as f32) as i32,
            (self.y + intended_velocity.1 as f32) as i32,
        );

        interactive_entities.retain(|entity| {
            entity.is_inside_bounds(intended_position.clone(), self.width, self.height)
        });

        if interactive_entities.len() > 0 {
            let left_collided_entity = interactive_entities
                .clone()
                .into_iter()
                .find(|entity| entity.x >= self.x + self.width as f32);
            if let Some(entity) = left_collided_entity {
                self.x = entity.x - self.width as f32;
                self.velocity_x = self.velocity_x * -1.0 * self.bouncyness * entity.bouncyness;
            } else {
                let right_collided_entity = interactive_entities
                    .clone()
                    .into_iter()
                    .find(|entity| entity.x + entity.width as f32 <= self.x);
                if let Some(entity) = right_collided_entity {
                    self.x = entity.x + entity.width as f32;
                    self.velocity_x = self.velocity_x * -1.0 * self.bouncyness * entity.bouncyness;
                } else {
                    self.x = intended_position.0 as f32;
                    self.velocity_x = intended_velocity.0;
                }
            }

            let top_collided_entity = interactive_entities
                .clone()
                .into_iter()
                .find(|entity| entity.y >= self.y + self.height as f32);
            if let Some(entity) = top_collided_entity {
                self.y = entity.y - self.height as f32;
                self.velocity_y = self.velocity_y * -1.0 * self.bouncyness * entity.bouncyness;
            } else {
                let bottom_collided_entity = interactive_entities
                    .clone()
                    .into_iter()
                    .find(|entity| entity.y + entity.height as f32 <= self.y);
                if let Some(entity) = bottom_collided_entity {
                    self.y = entity.y + entity.height as f32;
                    self.velocity_y = self.velocity_y * -1.0 * self.bouncyness * entity.bouncyness;
                } else {
                    self.y = intended_position.1 as f32;
                    self.velocity_y = intended_velocity.1;
                }
            }
        } else {
            self.velocity_x += self.acceleration_x;
            self.velocity_y += self.acceleration_y;

            self.x += self.velocity_x;
            self.y += self.velocity_y;
        }
    }
    fn is_inside_bounds(&self, position: (i32, i32), width: u16, height: u16) -> bool {
        (self.x as i32 + self.width as i32 >= position.0
            && self.y as i32 + self.height as i32 >= position.1
            && self.x as i32 <= position.0 + width as i32
            && self.y as i32 <= position.1 + height as i32)
            || (
                // for entities bigger than the rect
                (self.width > width && self.height > height)
                    && ((self.x as i32) < position.0 && (self.y as i32) < position.0)
                    && (self.x as i32 + self.width as i32 > width as i32
                        && self.y as i32 + self.height as i32 > height as i32)
            )
    }
    pub fn is_inside_rect(&self, rect: Rect) -> bool {
        (rect.x() + self.width as i32) >= 0
            && (rect.y() + self.height as i32) >= 0
            && rect.x() <= rect.width() as i32
            && rect.y() <= rect.height() as i32
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
    let mut camera = Camera::new(900, 600);
    camera.position.0 = -450.0;
    camera.position.1 = -300.0;

    let entity = Entity::new(0.0, 0.0, 10, 10);
    let (x, y, width, height) = entity.to_canvas_coordinates(&camera);
    let entity_like_at_the_beginning =
        Entity::from_canvas_coordinates((x, y, width, height, 1.0, 1.0), &camera);

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
fn window_entity_coordinates_vs_actual_coordinates_entity_at_start() {
    let camera = Camera::new(900, 600);

    let entity = Entity::new(0.0, 0.0, 10, 10);
    let (x, y, width, height) = entity.to_canvas_coordinates(&camera);
    let entity_like_at_the_beginning =
        Entity::from_canvas_coordinates((x, y, width, height, 1.0, 1.0), &camera);

    assert_eq!(x, 0.0);
    assert_eq!(y, 0.0);
    assert_eq!(width, 10);
    assert_eq!(height, 10);

    assert_eq!(entity.x, entity_like_at_the_beginning.x);
    assert_eq!(entity.y, entity_like_at_the_beginning.y);
    assert_eq!(entity.width, entity_like_at_the_beginning.width);
    assert_eq!(entity.height, entity_like_at_the_beginning.height);
}

#[test]
fn window_entity_coordinates_vs_actual_coordinates() {
    let mut camera = Camera::new(900, 600);
    camera.position.0 = 450.0;

    let entity = Entity::new(600.0, 0.0, 10, 10);
    let (x, y, width, height) = entity.to_canvas_coordinates(&camera);
    let entity_like_at_the_beginning =
        Entity::from_canvas_coordinates((x, y, width, height, 1.0, 1.0), &camera);

    assert_eq!(x, 150.0);
    assert_eq!(y, 0.0);
    assert_eq!(width, 10);
    assert_eq!(height, 10);

    assert_eq!(entity.x, entity_like_at_the_beginning.x);
    assert_eq!(entity.y, entity_like_at_the_beginning.y);
    assert_eq!(entity.width, entity_like_at_the_beginning.width);
    assert_eq!(entity.height, entity_like_at_the_beginning.height);
}

#[test]
fn window_entity_coordinates_vs_actual_coordinates_plus_scale() {
    let mut camera = Camera::new(900, 600);
    camera.zoom(2.0);
    camera.position.0 = 450.0;

    let entity = Entity::new(600.0, 0.0, 10, 10);
    let (x, y, width, height) = entity.to_canvas_coordinates(&camera);
    let entity_like_at_the_beginning =
        Entity::from_canvas_coordinates((x, y, width, height, 1.0, 1.0), &camera);

    assert_eq!(x, 300.0);
    assert_eq!(y, 0.0);
    assert_eq!(width, 20);
    assert_eq!(height, 20);

    assert_eq!(entity.x, entity_like_at_the_beginning.x);
    assert_eq!(entity.y, entity_like_at_the_beginning.y);
    assert_eq!(entity.width, entity_like_at_the_beginning.width);
    assert_eq!(entity.height, entity_like_at_the_beginning.height);
}

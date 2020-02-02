use super::level::Entity;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Camera {
  pub position: (f32, f32),
  pub scale: (f32, f32),
  pub dimensions: (u16, u16),
}
impl Camera {
  pub fn new(dimensions: (u16, u16)) -> Self {
    Self { position: (0.0, 0.0), scale: (1.0, 1.0), dimensions }
  }
  fn restrict_zoom(&mut self) {
    if self.scale.0 > 25.0 {
      self.scale.0 = 25.0
    }
    if self.scale.1 > 25.0 {
      self.scale.1 = 25.0
    }
    if self.scale.0 < 0.01 {
      self.scale.0 = 0.01
    }
    if self.scale.1 < 0.01 {
      self.scale.1 = 0.01
    }
  }
  pub fn zoom(&mut self, scale: f32) {
    self.scale.0 *= scale;
    self.scale.1 *= scale;
    self.restrict_zoom();
  }
  pub fn set_zoom(&mut self, scale: f32) {
    self.scale.0 = scale;
    self.scale.1 = scale;
    self.restrict_zoom();
  }
  pub fn to_target(&mut self, target_camera: &Self, rate: (f32, f32)) {
    self.position.0 += (target_camera.position.0 - self.position.0) * rate.0;
    self.position.1 += (target_camera.position.1 - self.position.1) * rate.1;
    self.scale.0 += (target_camera.scale.0 - self.scale.0) * rate.0;
    self.scale.1 += (target_camera.scale.1 - self.scale.1) * rate.1;
  }
  pub fn draw_relatively(
    &self,
    canvas: &mut WindowCanvas,
    entities: &Vec<Entity>,
    texture: &Texture,
  ) {
    for entity in entities {
      let (_x, _y, _width, _height) = entity.to_canvas_coordinates(
        self,
        ((self.dimensions.0 / 2) as u32, (self.dimensions.1 / 2) as u32),
      );
      let (x, y, width, height) = (_x as i32, _y as i32, _width as u32, _height as u32);
      if x + width as i32 >= 0
        && y + height as i32 >= 0
        && x <= self.dimensions.0 as i32
        && y <= self.dimensions.1 as i32
      {
        let entity_rect = Rect::new(x, y, width, height);
        if let Some(sprite_rect) = entity.sprite_sheet_rect {
          canvas
            .copy_ex(
              texture,
              Some(Rect::new(sprite_rect.0, sprite_rect.1, sprite_rect.2, sprite_rect.3)),
              Some(entity_rect),
              0.0,
              None,
              false,
              false,
            )
            .unwrap();
        } else {
          let original_color = canvas.draw_color();
          canvas.set_draw_color(Color { r: 46, g: 50, b: 40, a: 0xff });
          canvas.fill_rect(entity_rect).unwrap();
          canvas.set_draw_color(Color { r: 67, g: 86, b: 63, a: 0xff });
          canvas.draw_rect(entity_rect).unwrap();
          canvas.set_draw_color(original_color);
        }
      }
    }
  }
}

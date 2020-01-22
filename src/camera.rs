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
  pub fn get_x(&self) -> f32 {
    self.position.0
  }
  pub fn get_y(&self) -> f32 {
    self.position.1
  }
  pub fn get_scale_x(&self) -> f32 {
    self.scale.0
  }
  pub fn get_scale_y(&self) -> f32 {
    self.scale.1
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
    let difference = (self.scale.0 / scale, self.scale.1 / scale);
    self.scale.0 = scale;
    self.scale.1 = scale;
    self.restrict_zoom();
    self.position.0 *= difference.0;
    self.position.1 *= difference.1;
  }
  pub fn to_target(&mut self, target_camera: &Self, rate: (f32, f32)) {
    self.position.0 += (target_camera.get_x() - self.get_x()) * rate.0;
    self.position.1 += (target_camera.get_y() - self.get_y()) * rate.1;
    self.scale.0 += (target_camera.get_scale_x() - self.get_scale_x()) * rate.0;
    self.scale.1 += (target_camera.get_scale_y() - self.get_scale_y()) * rate.1;
  }
}

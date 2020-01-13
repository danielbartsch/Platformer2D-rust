pub mod camera {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Camera {
        pub position: (i32, i32),
        pub scale: (f32, f32),
        width: u16,
        height: u16,
    }
    impl Camera {
        pub fn new(width: u16, height: u16) -> Camera {
            Camera {
                position: (0, 0),
                scale: (1.0, 1.0),
                width,
                height,
            }
        }
        pub fn get_x(&self) -> i32 {
            self.position.0
        }
        pub fn get_y(&self) -> i32 {
            self.position.1
        }
        pub fn get_scale_x(&self) -> f32 {
            self.scale.0
        }
        pub fn get_scale_y(&self) -> f32 {
            self.scale.1
        }
        pub fn zoom(&mut self, scale: f32) {
            self.scale.0 *= scale;
            self.scale.1 *= scale;
        }
        pub fn set_zoom(&mut self, scale: f32) {
            let difference = (self.scale.0 / scale, self.scale.1 / scale);
            self.scale.0 = scale;
            self.scale.1 = scale;
            self.position.0 = (self.position.0 as f32 * difference.0) as i32;
            self.position.1 = (self.position.1 as f32 * difference.1) as i32;
        }
        pub fn to_target(&mut self, target_camera: &Self, rate: f32) {
            self.position.0 += ((target_camera.get_x() - self.get_x()) as f32 * rate) as i32;
            self.position.1 += ((target_camera.get_y() - self.get_y()) as f32 * rate) as i32;
            self.scale.0 += (target_camera.get_scale_x() - self.get_scale_x()) * rate;
            self.scale.1 += (target_camera.get_scale_y() - self.get_scale_y()) * rate;
        }
    }
}

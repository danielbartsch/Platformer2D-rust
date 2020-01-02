pub mod camera {
    #[derive(Debug)]
    pub struct Camera {
        pub x: i32,
        pub y: i32,
        width: u16,
        height: u16,
    }
    impl Camera {
        pub fn new(width: u16, height: u16) -> Camera {
            Camera {
                x: 0,
                y: 0,
                width,
                height,
            }
        }
        pub fn to_target(&mut self, target_x: i32, target_y: i32, rate: f32) {
            self.x += ((target_x - self.x) as f32 * rate) as i32;
            self.y += ((target_y - self.y) as f32 * rate) as i32;
        }
    }
}

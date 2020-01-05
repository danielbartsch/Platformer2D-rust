pub mod camera {
    #[derive(Debug, Clone, Copy)]
    pub struct Point(pub i32, pub i32);

    #[derive(Debug)]
    pub struct Camera {
        pub position: Point,
        width: u16,
        height: u16,
    }
    impl Camera {
        pub fn new(width: u16, height: u16) -> Camera {
            Camera {
                position: Point(0, 0),
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
        pub fn to_target(&mut self, target_position: Point, rate: f32) {
            self.position.0 += ((target_position.0 - self.get_x()) as f32 * rate) as i32;
            self.position.1 += ((target_position.1 - self.get_y()) as f32 * rate) as i32;
        }
    }
}

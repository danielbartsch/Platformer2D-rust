#[path = "camera.rs"]
mod camera;

pub mod app {
    use super::camera::camera::Camera;
    use super::camera::camera::Point;
    use super::level::*;
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use sdl2::render::WindowCanvas;
    use std::collections::HashSet;
    use std::time::Duration;

    static BACKGROUND_COLOR: Color = Color {
        r: 60,
        g: 30,
        b: 60,
        a: 0xff,
    };

    macro_rules! draw_relatively {
        ($canvas: expr, $entities: expr, $camera: expr) => {
            if $entities.len() > 0 {
                for entity in $entities {
                    let x = ((entity.x - $camera.get_x()) as f32 * entity.parallax_x) as i32;
                    let y = ((entity.y - $camera.get_y()) as f32 * entity.parallax_y) as i32;

                    $canvas.set_draw_color(Color {
                        r: 180,
                        g: 130,
                        b: 130,
                        a: 0xff,
                    });
                    $canvas
                        .draw_line((x, y), (x + entity.width as i32, y))
                        .unwrap();
                    $canvas
                        .draw_line(
                            (x + entity.width as i32, y),
                            (x + entity.width as i32, y + entity.height as i32),
                        )
                        .unwrap();
                    $canvas
                        .draw_line(
                            (x + entity.width as i32, y + entity.height as i32),
                            (x, y + entity.height as i32),
                        )
                        .unwrap();
                    $canvas
                        .draw_line((x, y + entity.height as i32), (x, y))
                        .unwrap();
                }
                $canvas.set_draw_color(BACKGROUND_COLOR);
            }
        };
    }

    pub fn run() {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window_width = 900;
        let window_height = 600;

        let window = video_subsystem
            .window("Editor", window_width, window_height)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        video_subsystem.text_input().start();

        let mut level1 = Level {
            background: vec![
                Entity::new(50, 50, 0, -100).parallax_x(0.4).parallax_y(0.4),
                Entity::new(50, 50, 100, 50).parallax_x(0.5).parallax_y(0.5),
                Entity::new(50, 50, -100, 50)
                    .parallax_x(0.6)
                    .parallax_y(0.6),
            ],
            indestructible: vec![
                Entity::new(50, 50, -375, -200),
                Entity::new(50, 40, -330, -260),
                Entity::new(50, 35, -285, -200),
                Entity::new(50, 55, -225, -320),
                Entity::new(50, 30, -225, 0),
                Entity::new(50, 25, -180, -73),
                Entity::new(50, 30, -135, 0),
                Entity::new(50, 40, -115, -360),
                Entity::new(50, 45, -90, -73),
                Entity::new(500, 40, -250, -146),
                Entity::new(50, 50, -45, 0),
                Entity::new(50, 20, 0, -260),
                Entity::new(50, 50, 0, -73),
                Entity::new(50, 35, 45, 0),
                Entity::new(50, 50, 90, -73),
                Entity::new(50, 45, 115, -360),
                Entity::new(50, 55, 135, 0),
                Entity::new(50, 25, 180, -73),
                Entity::new(50, 50, 225, 0),
                Entity::new(50, 50, 225, -320),
                Entity::new(50, 55, 285, -200),
                Entity::new(50, 40, 330, -260),
                Entity::new(50, 30, 375, -200),
            ],
            destructible: vec![],
            enemies: vec![],
            main_character: vec![Entity::new(20, 80, 0, -580)],
            effects: vec![],
            foreground: vec![],
        };

        let mut camera = Camera::new(900, 600);

        let mut camera_target = Point(level1.main_character[0].x, level1.main_character[0].y);

        'running: loop {
            canvas.set_draw_color(BACKGROUND_COLOR);
            canvas.clear();

            let mut pressed_keys = HashSet::new();
            pressed_keys = event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    _ => {}
                }
            }

            if pressed_keys.contains(&Keycode::N) {
                level1.main_character[0].velocity_y = -5.0;
            }
            if pressed_keys.contains(&Keycode::D) {
                camera_target = Point(camera_target.0, level1.main_character[0].y - 200);
            } else if pressed_keys.contains(&Keycode::S) {
                camera_target = Point(camera_target.0, level1.main_character[0].y + 200);
                level1.main_character[0].velocity_y = 5.0;
            }
            if pressed_keys.contains(&Keycode::A) {
                camera_target = Point(level1.main_character[0].x - 200, camera_target.1);
                level1.main_character[0].velocity_x = -5.0;
            } else if pressed_keys.contains(&Keycode::H) {
                camera_target = Point(level1.main_character[0].x + 200, camera_target.1);
                level1.main_character[0].velocity_x = 5.0;
            }
            if pressed_keys.len() == 0 {
                camera_target = Point(level1.main_character[0].x, level1.main_character[0].y);
            }

            let mut entities = vec![];
            entities.extend(&level1.indestructible);
            entities.extend(&level1.destructible);
            entities.extend(&level1.enemies);

            level1.main_character[0].next_state(entities);

            camera.to_target(
                Point(
                    camera_target.0 - (window_width / 2) as i32,
                    camera_target.1 - (window_height / 2) as i32,
                ),
                0.1,
            );

            draw_relatively!(canvas, &level1.background, &camera);
            draw_relatively!(canvas, &level1.indestructible, &camera);
            draw_relatively!(canvas, &level1.destructible, &camera);
            draw_relatively!(canvas, &level1.enemies, &camera);
            draw_relatively!(canvas, &level1.main_character, &camera);
            draw_relatively!(canvas, &level1.effects, &camera);
            draw_relatively!(canvas, &level1.foreground, &camera);

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 32));
        }
    }
}

pub mod level {
    use super::camera::camera::Point;

    pub enum EntityVariant {
        Block,
        Platform,
        MainCharacter,
    }
    pub struct Entity {
        pub variant: EntityVariant,
        pub width: u16,
        pub height: u16,
        pub x: i32,
        pub y: i32,
        pub velocity_x: f32,
        pub velocity_y: f32,
        pub acceleration_x: f32,
        pub acceleration_y: f32,
        pub parallax_x: f32,
        pub parallax_y: f32,
    }
    struct PointF32(f32, f32);
    impl Entity {
        pub fn new(width: u16, height: u16, x: i32, y: i32) -> Entity {
            Entity {
                variant: EntityVariant::Platform,
                width,
                height,
                x,
                y,
                velocity_x: 0.0,
                velocity_y: 0.0,
                acceleration_x: 0.0,
                acceleration_y: 0.0,
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
        pub fn next_state(&mut self, mut interactive_entities: Vec<&Entity>) {
            self.acceleration_x += 0.0;
            self.acceleration_y += 0.001;

            let intended_velocity = PointF32(
                self.velocity_x + self.acceleration_x,
                self.velocity_y + self.acceleration_y,
            );
            let intended_position = Point(
                self.x + intended_velocity.0 as i32,
                self.y + intended_velocity.1 as i32,
            );

            interactive_entities.retain(|entity| {
                entity.is_inside_bounds(intended_position.clone(), self.width, self.height)
            });

            if interactive_entities.len() > 0 {
                let left_collided_entity = interactive_entities
                    .clone()
                    .into_iter()
                    .find(|entity| entity.x >= self.x + self.width as i32);
                if let Some(entity) = left_collided_entity {
                    self.x = entity.x - self.width as i32;
                    self.velocity_x = 0.0;
                } else {
                    let right_collided_entity = interactive_entities
                        .clone()
                        .into_iter()
                        .find(|entity| entity.x + entity.width as i32 <= self.x);
                    if let Some(entity) = right_collided_entity {
                        self.x = entity.x + entity.width as i32;
                        self.velocity_x = 0.0;
                    } else {
                        self.x = intended_position.0;
                        self.velocity_x = intended_velocity.0;
                    }
                }

                let top_collided_entity = interactive_entities
                    .clone()
                    .into_iter()
                    .find(|entity| entity.y >= self.y + self.height as i32);
                if let Some(entity) = top_collided_entity {
                    self.y = entity.y - self.height as i32;
                    self.velocity_y = 0.0;
                } else {
                    let bottom_collided_entity = interactive_entities
                        .clone()
                        .into_iter()
                        .find(|entity| entity.y + entity.height as i32 <= self.y);
                    if let Some(entity) = bottom_collided_entity {
                        self.y = entity.y + entity.height as i32;
                        self.velocity_y = 0.0;
                    } else {
                        self.y = intended_position.1;
                        self.velocity_y = intended_velocity.1;
                    }
                }
            } else {
                self.velocity_x += self.acceleration_x;
                self.velocity_y += self.acceleration_y;

                self.x += self.velocity_x as i32;
                self.y += self.velocity_y as i32;
            }
        }
        pub fn is_inside_bounds(&self, position: Point, width: u16, height: u16) -> bool {
            (self.x + self.width as i32 >= position.0
                && self.y + self.height as i32 >= position.1
                && self.x <= position.0 + width as i32
                && self.y <= position.1 + height as i32)
                || (
                    // for entities bigger than the rect
                    (self.width > width && self.height > height)
                        && (self.x < position.0 && self.y < position.0)
                        && (self.x + self.width as i32 > width as i32
                            && self.y + self.height as i32 > height as i32)
                )
        }
    }
    pub struct Level {
        pub background: Vec<Entity>,
        pub indestructible: Vec<Entity>,
        pub destructible: Vec<Entity>,
        pub enemies: Vec<Entity>,
        pub main_character: Vec<Entity>,
        pub effects: Vec<Entity>,
        pub foreground: Vec<Entity>,
    }
}

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
        r: 30,
        g: 30,
        b: 30,
        a: 0xff,
    };

    macro_rules! draw_relatively {
        ($canvas: expr, $entities: expr, $camera: expr) => {
            if $entities.len() > 0 {
                for entity in $entities {
                    let x = ((entity.x - $camera.get_x()) as f32 * entity.parallax_x) as i32;
                    let y = ((entity.y - $camera.get_y()) as f32 * entity.parallax_y) as i32;

                    $canvas.set_draw_color(Color {
                        r: 80,
                        g: 30,
                        b: 30,
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
                Entity::new(50, 10, -135, -146),
                Entity::new(50, 30, -135, 0),
                Entity::new(50, 40, -115, -360),
                Entity::new(50, 45, -90, -73),
                Entity::new(50, 40, -45, -146),
                Entity::new(50, 50, -45, 0),
                Entity::new(50, 20, 0, -260),
                Entity::new(50, 50, 0, -73),
                Entity::new(50, 50, 45, -146),
                Entity::new(50, 35, 45, 0),
                Entity::new(50, 50, 90, -73),
                Entity::new(50, 45, 115, -360),
                Entity::new(50, 55, 135, 0),
                Entity::new(50, 50, 135, -146),
                Entity::new(50, 25, 180, -73),
                Entity::new(50, 50, 225, 0),
                Entity::new(50, 50, 225, -320),
                Entity::new(50, 55, 285, -200),
                Entity::new(50, 40, 330, -260),
                Entity::new(50, 30, 375, -200),
            ],
            destructible: vec![],
            enemies: vec![],
            main_character: vec![Entity::new(20, 80, 0, -80)],
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

            if pressed_keys.contains(&Keycode::D) {
                camera_target = Point(camera_target.0, level1.main_character[0].y - 200);
            } else if pressed_keys.contains(&Keycode::S) {
                camera_target = Point(camera_target.0, level1.main_character[0].y + 200);
            }
            if pressed_keys.contains(&Keycode::A) {
                camera_target = Point(level1.main_character[0].x - 200, camera_target.1);
            } else if pressed_keys.contains(&Keycode::H) {
                camera_target = Point(level1.main_character[0].x + 200, camera_target.1);
            }
            if pressed_keys.len() == 0 {
                camera_target = Point(level1.main_character[0].x, level1.main_character[0].y);
            }

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
        pub fn next_state(mut self) -> Self {
            self.acceleration_x += 0.0;
            self.acceleration_y += 0.001;

            self.velocity_x += self.acceleration_x;
            self.velocity_y += self.acceleration_y;

            self.x += self.velocity_x as i32;
            self.y += self.velocity_y as i32;

            self
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

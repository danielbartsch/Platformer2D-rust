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
                    match entity.variant {
                        EntityVariant::MainCharacter => {
                            $canvas
                                .draw_line((x, y + 4), (x + entity.width as i32, y + 4))
                                .unwrap();
                        }
                        EntityVariant::Platform => {
                            $canvas
                                .draw_line((x + 4, y), (x + 4, y + entity.height as i32))
                                .unwrap();
                            $canvas
                                .draw_line(
                                    (x + entity.width as i32 - 4, y),
                                    (x + entity.width as i32 - 4, y + entity.height as i32),
                                )
                                .unwrap();
                            $canvas
                                .draw_line(
                                    (
                                        x + entity.width as i32 - entity.width as i32 / 8,
                                        y + entity.height as i32 / 8,
                                    ),
                                    (
                                        x + entity.width as i32 / 8,
                                        y + entity.height as i32 - entity.height as i32 / 8,
                                    ),
                                )
                                .unwrap();
                            $canvas
                                .draw_line(
                                    (
                                        x + entity.width as i32 - entity.width as i32 / 8,
                                        y + entity.height as i32 - entity.height as i32 / 8,
                                    ),
                                    (x + entity.width as i32 / 8, y + entity.height as i32 / 8),
                                )
                                .unwrap();
                        }
                        EntityVariant::Pillar => {
                            for running_x in 0..entity.width / 4 {
                                let real_x = running_x as i32 * 4;
                                $canvas
                                    .draw_line(
                                        (x + real_x, y),
                                        (x + real_x, y + entity.height as i32),
                                    )
                                    .unwrap();
                                $canvas
                                    .draw_line(
                                        (x + entity.width as i32 - real_x, y),
                                        (
                                            x + entity.width as i32 - real_x,
                                            y + entity.height as i32,
                                        ),
                                    )
                                    .unwrap();
                            }
                        }
                        _ => {}
                    }
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

        let temple = vec![
            Entity::new(20, 300, -180, -330)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, -160, -330)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, -140, -330)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, -120, -330)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, -100, -330)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, -60, -330)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, -40, -330)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, -20, -330)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, 0, -330)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, 20, -330)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(460, 20, -190, -350).parallax_x(2.0),
            Entity::new(300, 20, -150, -370).parallax_x(2.0),
            Entity::new(40, 20, -85, -390).parallax_x(2.0),
        ];

        let small_temple = vec![
            Entity::new(5, 60, -180, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, -160, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, -140, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, -120, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, -100, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, -60, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, -40, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, -20, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, 0, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, 20, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(115, 5, -190, -435).parallax_x(0.5),
            Entity::new(75, 5, -150, -440).parallax_x(0.5),
            Entity::new(10, 5, -85, -445).parallax_x(0.5),
        ];

        let mut level1 = Level {
            background: small_temple
                .clone()
                .into_iter()
                .chain(
                    small_temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x - 300,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    small_temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 300,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    small_temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 600,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    small_temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 900,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    small_temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 1200,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    small_temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 1500,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    small_temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 1800,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    small_temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 2100,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    small_temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 2400,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(vec![Entity::new(5, 5, 0, -375).parallax_x(0.5)])
                .chain(vec![Entity::new(8, 8, 0, -357).parallax_x(0.5625)])
                .chain(vec![Entity::new(10, 10, 0, -338).parallax_x(0.625)])
                .chain(vec![Entity::new(15, 15, 0, -301).parallax_x(0.75)])
                .chain(vec![Entity::new(25, 25, 0, -227).parallax_x(1.0)])
                .chain(vec![Entity::new(38, 38, 0, -154).parallax_x(1.5)])
                .chain(vec![Entity::new(44, 44, 0, -117).parallax_x(1.75)])
                .chain(vec![Entity::new(47, 47, 0, -99).parallax_x(1.875)])
                .chain(vec![Entity::new(50, 50, 0, -80).parallax_x(2.0)])
                .collect::<Vec<Entity>>(),
            indestructible: vec![Entity::new(5000, 40, -2250, -146)],
            destructible: vec![],
            enemies: vec![],
            main_character: vec![Entity::new(20, 80, 0, -250).variant(EntityVariant::MainCharacter)],
            effects: vec![],
            foreground: temple
                .clone()
                .into_iter()
                .chain(
                    temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x - 300,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 300,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 600,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 900,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 1200,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 1500,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 1800,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 2100,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 2400,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .collect::<Vec<Entity>>(),
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

    #[derive(Clone)]
    pub enum EntityVariant {
        Block,
        Platform,
        MainCharacter,
        Pillar,
    }
    #[derive(Clone)]
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
        pub fn variant(mut self, variant: EntityVariant) -> Entity {
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

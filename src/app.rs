#[path = "camera.rs"]
mod camera;

pub mod app {
    use super::camera::camera::Camera;
    use super::camera::camera::Point;
    use super::level::*;
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use sdl2::render::WindowCanvas;
    use std::collections::HashSet;
    use std::time::Duration;

    static BACKGROUND_COLOR: Color = Color {
        r: 42,
        g: 43,
        b: 37,
        a: 0xff,
    };

    static LINE_COLOR: Color = Color {
        r: 67,
        g: 86,
        b: 63,
        a: 0xff,
    };
    static LINE_BACKGROUND_COLOR: Color = Color {
        r: 46,
        g: 50,
        b: 40,
        a: 0xff,
    };

    static MAIN_BACKGROUND_COLOR: Color = Color {
        r: 179,
        g: 54,
        b: 57,
        a: 0xff,
    };

    static MAIN_LINE_COLOR: Color = Color {
        r: 255,
        g: 128,
        b: 131,
        a: 0xff,
    };

    static WINDOW_WIDTH: u16 = 900;
    static WINDOW_HEIGHT: u16 = 600;

    macro_rules! draw_relatively {
        ($canvas: expr, $entities: expr, $camera: expr) => {
            if $entities.len() > 0 {
                for entity in $entities {
                    let x = entity.x
                        - ($camera.get_x() as f32 * entity.parallax_x - WINDOW_WIDTH as f32 / 2.0)
                            as i32;
                    let y = entity.y
                        - ($camera.get_y() as f32 * entity.parallax_y - WINDOW_HEIGHT as f32 / 2.0)
                            as i32;

                    if (x + entity.width as i32) >= 0
                        && (y + entity.height as i32) >= 0
                        && x <= WINDOW_WIDTH as i32
                        && y <= WINDOW_HEIGHT as i32
                    {
                        match entity.variant {
                            EntityVariant::MainCharacter => {
                                $canvas.set_draw_color(MAIN_BACKGROUND_COLOR);
                                $canvas
                                    .fill_rect(Rect::new(
                                        x,
                                        y,
                                        entity.width as u32,
                                        entity.height as u32,
                                    ))
                                    .unwrap();
                                $canvas.set_draw_color(MAIN_LINE_COLOR);
                                $canvas
                                    .draw_rect(Rect::new(
                                        x,
                                        y,
                                        entity.width as u32,
                                        entity.height as u32,
                                    ))
                                    .unwrap();
                                $canvas
                                    .draw_line((x, y + 4), (x + entity.width as i32, y + 4))
                                    .unwrap();
                            }
                            EntityVariant::Platform => {
                                $canvas.set_draw_color(LINE_BACKGROUND_COLOR);
                                $canvas
                                    .fill_rect(Rect::new(
                                        x,
                                        y,
                                        entity.width as u32,
                                        entity.height as u32,
                                    ))
                                    .unwrap();
                                $canvas.set_draw_color(LINE_COLOR);
                                $canvas
                                    .draw_rect(Rect::new(
                                        x,
                                        y,
                                        entity.width as u32,
                                        entity.height as u32,
                                    ))
                                    .unwrap();
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
                                $canvas.set_draw_color(LINE_BACKGROUND_COLOR);
                                $canvas
                                    .fill_rect(Rect::new(
                                        x,
                                        y,
                                        entity.width as u32,
                                        entity.height as u32,
                                    ))
                                    .unwrap();
                                $canvas.set_draw_color(LINE_COLOR);
                                $canvas
                                    .draw_rect(Rect::new(
                                        x,
                                        y,
                                        entity.width as u32,
                                        entity.height as u32,
                                    ))
                                    .unwrap();
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
                        $canvas.set_draw_color(BACKGROUND_COLOR);
                    }
                }
            }
        };
    }

    pub fn run() {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Platformer 2D", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        video_subsystem.text_input().start();

        let temple = vec![
            Entity::new(20, 300, 40, -230)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, 120, -230)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, 200, -230)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, 280, -230)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, 360, -230)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, 520, -230)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, 600, -230)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, 680, -230)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, 760, -230)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(20, 300, 840, -230)
                .parallax_x(2.0)
                .variant(EntityVariant::Pillar),
            Entity::new(900, 20, 0, -250).parallax_x(2.0),
            Entity::new(480, 20, 160, -270).parallax_x(2.0),
            Entity::new(60, 20, 420, -290).parallax_x(2.0),
        ];

        let small_temple = vec![
            Entity::new(5, 60, 10, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, 30, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, 50, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, 70, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, 90, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, 130, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, 150, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, 170, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, 190, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(5, 60, 210, -430)
                .parallax_x(0.5)
                .variant(EntityVariant::Pillar),
            Entity::new(225, 5, 0, -435).parallax_x(0.5),
            Entity::new(145, 5, 40, -440).parallax_x(0.5),
            Entity::new(15, 5, 105, -445).parallax_x(0.5),
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
                .chain(
                    small_temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 2700,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(vec![Entity::new(5, 5, 0, -375).parallax_x(0.5)])
                .chain(vec![Entity::new(10, 10, 0, -289).parallax_x(0.65)])
                .chain(vec![Entity::new(15, 15, 0, -242).parallax_x(0.8)])
                .chain(vec![Entity::new(20, 20, 0, -205).parallax_x(0.95)])
                .chain(vec![Entity::new(25, 25, 0, -177).parallax_x(1.0)])
                .chain(vec![Entity::new(30, 30, 0, -140).parallax_x(1.25)])
                .chain(vec![Entity::new(35, 35, 0, -102).parallax_x(1.4)])
                .chain(vec![Entity::new(40, 40, 0, -75).parallax_x(1.55)])
                .chain(vec![Entity::new(45, 45, 0, -37).parallax_x(1.7)])
                .chain(vec![Entity::new(50, 50, 0, 0).parallax_x(1.95)])
                .chain(vec![Entity::new(5, 5, 300, -375).parallax_x(0.5)])
                .chain(vec![Entity::new(10, 10, 300, -289).parallax_x(0.65)])
                .chain(vec![Entity::new(15, 15, 300, -242).parallax_x(0.8)])
                .chain(vec![Entity::new(20, 20, 300, -205).parallax_x(0.95)])
                .chain(vec![Entity::new(25, 25, 300, -177).parallax_x(1.0)])
                .chain(vec![Entity::new(30, 30, 300, -140).parallax_x(1.25)])
                .chain(vec![Entity::new(35, 35, 300, -102).parallax_x(1.4)])
                .chain(vec![Entity::new(40, 40, 300, -75).parallax_x(1.55)])
                .chain(vec![Entity::new(45, 45, 300, -37).parallax_x(1.7)])
                .chain(vec![Entity::new(50, 50, 300, 0).parallax_x(1.95)])
                .chain(vec![Entity::new(5, 5, 600, -375).parallax_x(0.5)])
                .chain(vec![Entity::new(10, 10, 600, -289).parallax_x(0.65)])
                .chain(vec![Entity::new(15, 15, 600, -242).parallax_x(0.8)])
                .chain(vec![Entity::new(20, 20, 600, -205).parallax_x(0.95)])
                .chain(vec![Entity::new(25, 25, 600, -177).parallax_x(1.0)])
                .chain(vec![Entity::new(30, 30, 600, -140).parallax_x(1.25)])
                .chain(vec![Entity::new(35, 35, 600, -102).parallax_x(1.4)])
                .chain(vec![Entity::new(40, 40, 600, -75).parallax_x(1.55)])
                .chain(vec![Entity::new(45, 45, 600, -37).parallax_x(1.7)])
                .chain(vec![Entity::new(50, 50, 600, 0).parallax_x(1.95)])
                .chain(vec![Entity::new(5, 5, 900, -375).parallax_x(0.5)])
                .chain(vec![Entity::new(10, 10, 900, -289).parallax_x(0.65)])
                .chain(vec![Entity::new(15, 15, 900, -242).parallax_x(0.8)])
                .chain(vec![Entity::new(20, 20, 900, -205).parallax_x(0.95)])
                .chain(vec![Entity::new(25, 25, 900, -177).parallax_x(1.0)])
                .chain(vec![Entity::new(30, 30, 900, -140).parallax_x(1.25)])
                .chain(vec![Entity::new(35, 35, 900, -102).parallax_x(1.4)])
                .chain(vec![Entity::new(40, 40, 900, -75).parallax_x(1.55)])
                .chain(vec![Entity::new(45, 45, 900, -37).parallax_x(1.7)])
                .chain(vec![Entity::new(50, 50, 900, 0).parallax_x(1.95)])
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
                            x: entity.x + 2400,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 3600,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 4800,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 6000,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 7200,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 8400,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 9600,
                            ..entity
                        })
                        .collect::<Vec<Entity>>(),
                )
                .chain(
                    temple
                        .clone()
                        .into_iter()
                        .map(|entity| Entity {
                            x: entity.x + 10800,
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

            let mut entities = vec![];
            entities.extend(&level1.indestructible);
            entities.extend(&level1.destructible);
            entities.extend(&level1.enemies);

            if pressed_keys.contains(&Keycode::N)
                && level1.main_character[0].is_touching_ground(entities.clone())
            {
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
            } else {
                level1.main_character[0].velocity_x *= 0.8;
            }
            if pressed_keys.len() == 0 {
                camera_target = Point(level1.main_character[0].x, level1.main_character[0].y);
            }

            level1.main_character[0].next_state(entities);

            camera.to_target(Point(camera_target.0, camera_target.1), 0.1);

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
        pub fn is_touching_ground(&mut self, interactive_entities: Vec<&Entity>) -> bool {
            let lower_end = self.y + self.height as i32;
            interactive_entities.iter().any(|entity| {
                entity.y == lower_end
                    && ((entity.x < self.x && entity.x + entity.width as i32 > self.x)
                        || (entity.x < self.x + self.width as i32
                            && entity.x + entity.width as i32 > self.x + self.width as i32))
            })
        }
        pub fn next_state(&mut self, mut interactive_entities: Vec<&Entity>) {
            self.acceleration_x = 0.0;
            self.acceleration_y = 0.3;

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

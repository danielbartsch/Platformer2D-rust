#[path = "camera.rs"]
mod camera;

pub mod app {
    use super::camera::camera::Camera;
    use super::camera::camera::Point;
    use super::editor_menu::EditorMenu;
    use super::level::*;
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use sdl2::render::WindowCanvas;
    use std::cmp;
    use std::collections::HashSet;
    use std::fs;
    use std::time::{Duration, SystemTime};

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

    static MAX_FRAME_TIME_MILLIS: i8 = 16;

    macro_rules! draw_relatively {
        ($canvas: expr, $entities: expr, $camera: expr) => {
            if $entities.len() > 0 {
                for entity in $entities {
                    let x = entity.x as i32
                        - ($camera.get_x() as f32 * entity.parallax_x - WINDOW_WIDTH as f32 / 2.0)
                            as i32;
                    let y = entity.y as i32
                        - ($camera.get_y() as f32 * entity.parallax_y - WINDOW_HEIGHT as f32 / 2.0)
                            as i32;

                    if (x + entity.width as i32) >= 0
                        && (y + entity.height as i32) >= 0
                        && x <= WINDOW_WIDTH as i32
                        && y <= WINDOW_HEIGHT as i32
                    {
                        match entity.variant {
                            EntityVariant::Block => {
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
                            }
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

        let mut level1 = Level::deserialize(fs::read_to_string("levels/temples.json").unwrap());

        let mut camera = Camera::new(900, 600);

        let mut character_index = 0;

        let mut last_frame_time = SystemTime::now();

        let mut mouse_click_position = None;
        let mut mouse_selection_rect: Option<Rect> = None;

        'running: loop {
            canvas.set_draw_color(BACKGROUND_COLOR);
            canvas.clear();

            let mut pressed_keys = HashSet::new();
            pressed_keys = event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();

            let mouse_state = event_pump.mouse_state();

            let (mouse_x, mouse_y) = (mouse_state.x(), mouse_state.y());

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::P),
                        ..
                    } => {
                        character_index = (character_index + 1) % level1.main_character.len();
                    }
                    Event::MouseButtonDown { x, y, .. } => match mouse_click_position {
                        Some(_) => {
                            mouse_click_position = None;

                            let camera_to_level_coordinates = Some(Rect::new(
                                mouse_selection_rect.unwrap().x() + camera.position.0
                                    - WINDOW_WIDTH as i32 / 2,
                                mouse_selection_rect.unwrap().y() + camera.position.1
                                    - WINDOW_HEIGHT as i32 / 2,
                                mouse_selection_rect.unwrap().width(),
                                mouse_selection_rect.unwrap().height(),
                            ));

                            EditorMenu::create_entity(
                                &mut level1,
                                camera_to_level_coordinates.unwrap(),
                            );
                            mouse_selection_rect = None;
                        }
                        None => {
                            mouse_click_position = Some((x, y));
                        }
                    },
                    _ => {}
                }
            }

            let mut entities = vec![];
            entities.extend(&level1.indestructible);
            entities.extend(&level1.destructible);
            entities.extend(&level1.enemies);

            if pressed_keys.contains(&Keycode::Y) {
                let pseudo_random = last_frame_time.elapsed().unwrap().as_nanos() as f32;
                level1.effects.push(
                    Entity::new(
                        10,
                        10,
                        level1.main_character[character_index].x,
                        level1.main_character[character_index].y,
                    )
                    .variant(EntityVariant::Block)
                    .velocity_x(
                        level1.main_character[character_index].velocity_x
                            * (2.2 + pseudo_random.cos()),
                    )
                    .velocity_y(
                        level1.main_character[character_index].velocity_y
                            * (2.2 + pseudo_random.sin()),
                    )
                    .acceleration_y(0.01)
                    .bouncyness(1.1),
                );
            }
            if pressed_keys.contains(&Keycode::N)
                && level1.main_character[character_index].is_touching_ground(entities.clone())
            {
                level1.main_character[character_index].velocity_y = -5.0;
            }
            if pressed_keys.contains(&Keycode::D) {
                level1.cameras[0].position.1 =
                    level1.main_character[character_index].y as i32 - 400;
            } else if pressed_keys.contains(&Keycode::S) {
                level1.cameras[0].position.1 =
                    level1.main_character[character_index].y as i32 + 400;
                level1.main_character[character_index].velocity_y = 5.0;
            }
            if pressed_keys.contains(&Keycode::A) {
                level1.cameras[0].position.0 =
                    level1.main_character[character_index].x as i32 - 400;
                level1.main_character[character_index].velocity_x = -5.0;
            } else if pressed_keys.contains(&Keycode::H) {
                level1.cameras[0].position.0 =
                    level1.main_character[character_index].x as i32 + 400;
                level1.main_character[character_index].velocity_x = 5.0;
            } else {
                level1.main_character[character_index].velocity_x *= 0.8;
            }
            if !pressed_keys.contains(&Keycode::A)
                && !pressed_keys.contains(&Keycode::S)
                && !pressed_keys.contains(&Keycode::H)
                && !pressed_keys.contains(&Keycode::D)
            {
                level1.cameras[0].position = Point(
                    level1.main_character[character_index].x as i32,
                    level1.main_character[character_index].y as i32,
                );
            }

            for character in &mut level1.main_character {
                character.next_state(entities.clone());
            }
            for character in &mut level1.effects {
                character.next_state(entities.clone());
            }

            camera.to_target(level1.cameras[0].position, 0.03);

            draw_relatively!(canvas, &level1.background, &camera);
            draw_relatively!(canvas, &level1.indestructible, &camera);
            draw_relatively!(canvas, &level1.destructible, &camera);
            draw_relatively!(canvas, &level1.enemies, &camera);
            draw_relatively!(canvas, &level1.main_character, &camera);
            draw_relatively!(canvas, &level1.effects, &camera);
            draw_relatively!(canvas, &level1.foreground, &camera);

            match mouse_click_position {
                Some((x, y)) => {
                    let original_color = canvas.draw_color();
                    canvas.set_draw_color(Color {
                        r: 255,
                        g: 0,
                        b: 0,
                        a: 0xff,
                    });
                    let (pos_x, width) =
                        (cmp::min(x, mouse_x), (x - mouse_x).wrapping_abs() as u32);
                    let (pos_y, height) =
                        (cmp::min(y, mouse_y), (y - mouse_y).wrapping_abs() as u32);

                    mouse_selection_rect = Some(Rect::new(pos_x, pos_y, width, height));
                    canvas.draw_rect(mouse_selection_rect.unwrap()).unwrap();
                    canvas.set_draw_color(original_color);
                }
                None => {}
            }

            canvas.present();

            let millis_to_sleep = MAX_FRAME_TIME_MILLIS as i32
                - last_frame_time.elapsed().unwrap().as_millis() as i32;
            if millis_to_sleep > 0 {
                ::std::thread::sleep(Duration::new(0, millis_to_sleep as u32 * 1_000_000u32));
            } else {
                println!("Detecting Lag. Frame took {}ms too long", -millis_to_sleep);
            }
            last_frame_time = SystemTime::now();
        }
    }
}

pub mod editor_menu {
    use super::level::{Entity, Level};
    use ::sdl2::messagebox::*;
    use sdl2::rect::Rect;
    pub struct EditorMenu {}
    impl EditorMenu {
        pub fn create_entity(level: &mut Level, mouse_rect: Rect) {
            match show_message_box(
                MessageBoxFlag::INFORMATION,
                vec![
                    ButtonData {
                        flags: MessageBoxButtonFlag::RETURNKEY_DEFAULT,
                        button_id: 1,
                        text: "Create Entity",
                    },
                    ButtonData {
                        flags: MessageBoxButtonFlag::NOTHING,
                        button_id: 2,
                        text: "Delete under selection",
                    },
                    ButtonData {
                        flags: MessageBoxButtonFlag::ESCAPEKEY_DEFAULT,
                        button_id: 3,
                        text: "Cancel",
                    },
                ]
                .as_slice(),
                "What to do?",
                "",
                None,
                None,
            ) {
                Ok(message_box_result) => match message_box_result {
                    ClickedButton::CustomButton(ButtonData { text, .. }) => match text {
                        &"Create Entity" => {
                            match show_message_box(
                                MessageBoxFlag::INFORMATION,
                                vec![
                                    ButtonData {
                                        flags: MessageBoxButtonFlag::RETURNKEY_DEFAULT,
                                        button_id: 1,
                                        text: "Indestructible",
                                    },
                                    ButtonData {
                                        flags: MessageBoxButtonFlag::NOTHING,
                                        button_id: 2,
                                        text: "Main Character",
                                    },
                                    ButtonData {
                                        flags: MessageBoxButtonFlag::ESCAPEKEY_DEFAULT,
                                        button_id: 5,
                                        text: "Cancel",
                                    },
                                ]
                                .as_slice(),
                                "What to create?",
                                "",
                                None,
                                None,
                            ) {
                                Ok(creation_message_box_result) => {
                                    match creation_message_box_result {
                                        ClickedButton::CustomButton(ButtonData {
                                            text, ..
                                        }) => match text {
                                            &"Indestructible" => {
                                                level.indestructible.push(Entity::new(
                                                    mouse_rect.width() as u16,
                                                    mouse_rect.height() as u16,
                                                    mouse_rect.x() as f32,
                                                    mouse_rect.y() as f32,
                                                ));
                                            }
                                            &"Main Character" => {
                                                level.main_character.push(Entity::new(
                                                    mouse_rect.width() as u16,
                                                    mouse_rect.height() as u16,
                                                    mouse_rect.x() as f32,
                                                    mouse_rect.y() as f32,
                                                ));
                                            }
                                            _ => {}
                                        },
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        &"Delete under selection" => {
                            match show_message_box(
                                MessageBoxFlag::INFORMATION,
                                vec![
                                    ButtonData {
                                        flags: MessageBoxButtonFlag::RETURNKEY_DEFAULT,
                                        button_id: 1,
                                        text: "Yes, delete them!",
                                    },
                                    ButtonData {
                                        flags: MessageBoxButtonFlag::ESCAPEKEY_DEFAULT,
                                        button_id: 2,
                                        text: "Cancel",
                                    },
                                ]
                                .as_slice(),
                                &format!("Are you sure you want to delete these {} entities?", 0),
                                "",
                                None,
                                None,
                            ) {
                                Ok(deletion_message_box_result) => {
                                    match deletion_message_box_result {
                                        ClickedButton::CustomButton(ButtonData {
                                            text, ..
                                        }) => match text {
                                            &"Yes, delete them!" => {}
                                            _ => {}
                                        },
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

pub mod level {
    use super::camera::camera::Camera;
    use super::camera::camera::Point;
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
    struct PointF32(f32, f32);
    impl Entity {
        pub fn new(width: u16, height: u16, x: f32, y: f32) -> Self {
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
                acceleration_y: 0.3,
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
        pub fn next_state(&mut self, mut interactive_entities: Vec<&Self>) {
            let intended_velocity = PointF32(
                self.velocity_x + self.acceleration_x,
                self.velocity_y + self.acceleration_y,
            );
            let intended_position = Point(
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
                        self.velocity_x =
                            self.velocity_x * -1.0 * self.bouncyness * entity.bouncyness;
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
                        self.velocity_y =
                            self.velocity_y * -1.0 * self.bouncyness * entity.bouncyness;
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
        pub fn is_inside_bounds(&self, position: Point, width: u16, height: u16) -> bool {
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
        pub cameras: Vec<Camera>,
        pub foreground: Vec<Entity>,
    }

    impl Level {
        pub fn serialize(&self) -> String {
            serde_json::to_string(&self).unwrap()
        }
        pub fn deserialize(serialized: String) -> Level {
            serde_json::from_str(&serialized).unwrap()
        }
    }
}

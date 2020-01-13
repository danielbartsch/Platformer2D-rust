#[path = "camera.rs"]
mod camera;

pub mod app {
    use super::camera::camera::Camera;
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
    use std::mem;
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
                    let x = (entity.x * $camera.get_scale_x()) as i32
                        - ($camera.get_x() as f32 * (entity.parallax_x * $camera.get_scale_x())
                            - WINDOW_WIDTH as f32 / 2.0) as i32;
                    let y = (entity.y * $camera.get_scale_y()) as i32
                        - ($camera.get_y() as f32 * (entity.parallax_y * $camera.get_scale_y())
                            - WINDOW_HEIGHT as f32 / 2.0) as i32;

                    let width = (entity.width as f32 * $camera.get_scale_x()) as i32;
                    let height = (entity.height as f32 * $camera.get_scale_y()) as i32;

                    if (x + width) >= 0
                        && (y + height) >= 0
                        && x <= WINDOW_WIDTH as i32
                        && y <= WINDOW_HEIGHT as i32
                    {
                        match entity.variant {
                            EntityVariant::Block => {
                                $canvas.set_draw_color(MAIN_BACKGROUND_COLOR);
                                $canvas
                                    .fill_rect(Rect::new(x, y, width as u32, height as u32))
                                    .unwrap();
                                $canvas.set_draw_color(MAIN_LINE_COLOR);
                                $canvas
                                    .draw_rect(Rect::new(x, y, width as u32, height as u32))
                                    .unwrap();
                            }
                            EntityVariant::MainCharacter => {
                                $canvas.set_draw_color(MAIN_BACKGROUND_COLOR);
                                $canvas
                                    .fill_rect(Rect::new(x, y, width as u32, height as u32))
                                    .unwrap();
                                $canvas.set_draw_color(MAIN_LINE_COLOR);
                                $canvas
                                    .draw_rect(Rect::new(x, y, width as u32, height as u32))
                                    .unwrap();
                                $canvas.draw_line((x, y + 4), (x + width, y + 4)).unwrap();
                            }
                            EntityVariant::Platform => {
                                $canvas.set_draw_color(LINE_BACKGROUND_COLOR);
                                $canvas
                                    .fill_rect(Rect::new(x, y, width as u32, height as u32))
                                    .unwrap();
                                $canvas.set_draw_color(LINE_COLOR);
                                $canvas
                                    .draw_rect(Rect::new(x, y, width as u32, height as u32))
                                    .unwrap();
                                $canvas.draw_line((x + 4, y), (x + 4, y + height)).unwrap();
                                $canvas
                                    .draw_line((x + width - 4, y), (x + width - 4, y + height))
                                    .unwrap();
                                $canvas
                                    .draw_line(
                                        (x + width - width / 8, y + height / 8),
                                        (x + width / 8, y + height - height / 8),
                                    )
                                    .unwrap();
                                $canvas
                                    .draw_line(
                                        (x + width - width / 8, y + height - height / 8),
                                        (x + width / 8, y + height / 8),
                                    )
                                    .unwrap();
                            }
                            EntityVariant::Pillar => {
                                $canvas.set_draw_color(LINE_BACKGROUND_COLOR);
                                $canvas
                                    .fill_rect(Rect::new(x, y, width as u32, height as u32))
                                    .unwrap();
                                $canvas.set_draw_color(LINE_COLOR);
                                $canvas
                                    .draw_rect(Rect::new(x, y, width as u32, height as u32))
                                    .unwrap();
                                for running_x in 0..width / 4 {
                                    let real_x = running_x as i32 * 4;
                                    $canvas
                                        .draw_line((x + real_x, y), (x + real_x, y + height))
                                        .unwrap();
                                    $canvas
                                        .draw_line(
                                            (x + width - real_x, y),
                                            (x + width - real_x, y + height),
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

        let mut level1 =
            Level::deserialize(fs::read_to_string("assets/levels/temples.json").unwrap());

        let mut camera = Camera::new(900, 600);

        let mut character_index = 0;

        let mut last_frame_time = SystemTime::now();

        let mut edit_mode = false;
        let mut editor_menu = EditorMenu::new();

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
                    Event::Quit { .. } => {
                        break 'running;
                    }
                    Event::KeyDown { keycode, .. } => match keycode {
                        Some(Keycode::Escape) => {
                            mouse_click_position = None;
                        }
                        Some(Keycode::P) => {
                            character_index = (character_index + 1) % level1.main_character.len();
                        }
                        Some(Keycode::Num0) => {
                            edit_mode = !edit_mode;
                        }
                        _ => {}
                    },
                    Event::MouseWheel { y, .. } => {
                        if edit_mode {
                            if y < 0 {
                                level1.cameras[0].zoom(0.97);
                            } else {
                                level1.cameras[0].zoom(1.03);
                            }
                        }
                    }
                    Event::MouseButtonDown { x, y, .. } => {
                        if edit_mode {
                            let clicked_variant_button = EditorMenu::get_variant_button_rects()
                                .into_iter()
                                .find(|(_, rect)| {
                                    x > rect.x()
                                        && x < rect.x() + rect.width() as i32
                                        && y > rect.y()
                                        && y < rect.y() + rect.height() as i32
                                });
                            let clicked_entity_variant_button =
                                EditorMenu::get_entity_variant_button_rects()
                                    .into_iter()
                                    .find(|(_, rect)| {
                                        x > rect.x()
                                            && x < rect.x() + rect.width() as i32
                                            && y > rect.y()
                                            && y < rect.y() + rect.height() as i32
                                    });
                            if mem::discriminant(&clicked_entity_variant_button)
                                == mem::discriminant(&None)
                                && mem::discriminant(&clicked_variant_button)
                                    == mem::discriminant(&None)
                            {
                                match mouse_click_position {
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

                                        editor_menu.create_entity(
                                            &mut level1,
                                            camera_to_level_coordinates.unwrap(),
                                        );
                                        mouse_selection_rect = None;
                                    }
                                    None => {
                                        mouse_click_position = Some((x, y));
                                    }
                                }
                            } else {
                                if let Some((variant, _)) = clicked_variant_button {
                                    editor_menu.variant(variant);
                                }
                                if let Some((variant, _)) = clicked_entity_variant_button {
                                    editor_menu.entity_variant(variant);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            let mut entities = vec![];
            entities.extend(&level1.indestructible);
            entities.extend(&level1.destructible);
            entities.extend(&level1.enemies);

            if edit_mode {
                if pressed_keys.contains(&Keycode::D) {
                    level1.cameras[0].position.1 -= 25;
                } else if pressed_keys.contains(&Keycode::S) {
                    level1.cameras[0].position.1 += 25;
                }
                if pressed_keys.contains(&Keycode::A) {
                    level1.cameras[0].position.0 -= 25;
                } else if pressed_keys.contains(&Keycode::H) {
                    level1.cameras[0].position.0 += 25;
                }
                if pressed_keys.contains(&Keycode::Q) {
                    level1.cameras[0].zoom(1.03);
                } else if pressed_keys.contains(&Keycode::R) {
                    level1.cameras[0].zoom(0.97);
                }
            } else {
                level1.cameras[0].set_zoom(1.0);
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
                if pressed_keys.contains(&Keycode::N) {
                    if level1.main_character[character_index].is_touching_ground(entities.clone()) {
                        level1.main_character[character_index].velocity_y = -8.0;
                        level1.main_character[character_index].acceleration_y = 0.1;
                    }
                    if level1.main_character[character_index].velocity_y < 0.0 {
                        level1.main_character[character_index].acceleration_y += 0.01;
                    } else {
                        level1.main_character[character_index].acceleration_y += 0.002;
                    }
                } else {
                    level1.main_character[character_index].acceleration_y = 1.0;
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
                    level1.cameras[0].position = (
                        level1.main_character[character_index].x as i32,
                        level1.main_character[character_index].y as i32,
                    );
                }
            }

            for character in &mut level1.main_character {
                character.next_state(entities.clone());
            }
            for character in &mut level1.effects {
                character.next_state(entities.clone());
            }

            camera.to_target(&level1.cameras[0], if edit_mode { 0.3 } else { 0.03 });

            draw_relatively!(canvas, &level1.background, &camera);
            draw_relatively!(canvas, &level1.indestructible, &camera);
            draw_relatively!(canvas, &level1.destructible, &camera);
            draw_relatively!(canvas, &level1.enemies, &camera);
            draw_relatively!(canvas, &level1.main_character, &camera);
            draw_relatively!(canvas, &level1.effects, &camera);
            draw_relatively!(canvas, &level1.foreground, &camera);

            if edit_mode {
                let original_color = canvas.draw_color();
                canvas.set_draw_color(Color {
                    r: 255,
                    g: 60,
                    b: 60,
                    a: 0xff,
                });

                canvas.draw_rect(Rect::new(0, 0, 20, 20)).unwrap();
                canvas.draw_line((5, 5), (20 - 5, 5)).unwrap();
                canvas.draw_line((5, 5), (5, 20 - 5)).unwrap();
                canvas.draw_line((5, 20 - 5), (20 - 5, 20 - 5)).unwrap();
                canvas.draw_line((5, 20 / 2), (20 - 5, 20 / 2)).unwrap();

                for (variant, rect) in EditorMenu::get_variant_button_rects() {
                    canvas.draw_rect(rect).unwrap();
                    if mem::discriminant(&variant) == mem::discriminant(&editor_menu.variant) {
                        canvas
                            .draw_rect(Rect::new(
                                rect.x() + 5,
                                rect.y() + 5,
                                rect.width() - 10,
                                rect.height() - 10,
                            ))
                            .unwrap();
                    }
                }
                for (variant, rect) in EditorMenu::get_entity_variant_button_rects() {
                    canvas.draw_rect(rect).unwrap();
                    if mem::discriminant(&variant) == mem::discriminant(&editor_menu.entity_variant)
                    {
                        canvas
                            .draw_rect(Rect::new(
                                rect.x() + 5,
                                rect.y() + 5,
                                rect.width() - 10,
                                rect.height() - 10,
                            ))
                            .unwrap();
                    }
                }

                canvas.set_draw_color(original_color);

                match mouse_click_position {
                    Some((x, y)) => {
                        let original_color = canvas.draw_color();
                        canvas.set_draw_color(Color {
                            r: 255,
                            g: 60,
                            b: 60,
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
    use super::level::{Entity, EntityVariant, Level};
    use sdl2::rect::Rect;

    #[derive(Debug)]
    pub enum LevelEntityVariant {
        Background,
        Indestructible,
        Destructible,
        Enemies,
        MainCharacter,
        Effects,
        Foreground,
        Deletion,
    }
    pub struct EditorMenu {
        pub variant: LevelEntityVariant,
        pub entity_variant: EntityVariant,
    }
    impl EditorMenu {
        pub fn new() -> EditorMenu {
            EditorMenu {
                variant: LevelEntityVariant::Effects,
                entity_variant: EntityVariant::Platform,
            }
        }
        pub fn variant(&mut self, variant: LevelEntityVariant) {
            self.variant = variant;
        }
        pub fn entity_variant(&mut self, entity_variant: EntityVariant) {
            self.entity_variant = entity_variant;
        }
        pub fn get_variant_button_rects() -> Vec<(LevelEntityVariant, Rect)> {
            vec![
                (LevelEntityVariant::Background, Rect::new(0, 30, 20, 20)),
                (
                    LevelEntityVariant::Indestructible,
                    Rect::new(0, 30 + (25), 20, 20),
                ),
                (
                    LevelEntityVariant::Destructible,
                    Rect::new(0, 30 + (25 * 2), 20, 20),
                ),
                (
                    LevelEntityVariant::Enemies,
                    Rect::new(0, 30 + (25 * 3), 20, 20),
                ),
                (
                    LevelEntityVariant::MainCharacter,
                    Rect::new(0, 30 + (25 * 4), 20, 20),
                ),
                (
                    LevelEntityVariant::Effects,
                    Rect::new(0, 30 + (25 * 5), 20, 20),
                ),
                (
                    LevelEntityVariant::Foreground,
                    Rect::new(0, 30 + (25 * 6), 20, 20),
                ),
                (
                    LevelEntityVariant::Deletion,
                    Rect::new(0, 30 + (25 * 7), 20, 20),
                ),
            ]
        }
        pub fn get_entity_variant_button_rects() -> Vec<(EntityVariant, Rect)> {
            vec![
                (EntityVariant::Block, Rect::new(30, 30, 20, 20)),
                (EntityVariant::Platform, Rect::new(30, 30 + (25), 20, 20)),
                (
                    EntityVariant::MainCharacter,
                    Rect::new(30, 30 + (25 * 2), 20, 20),
                ),
                (EntityVariant::Pillar, Rect::new(30, 30 + (25 * 3), 20, 20)),
            ]
        }
        pub fn create_entity(&mut self, level: &mut Level, mouse_rect: Rect) {
            let entity = Entity::new(
                mouse_rect.width() as u16,
                mouse_rect.height() as u16,
                mouse_rect.x() as f32,
                mouse_rect.y() as f32,
            )
            .variant(self.entity_variant.clone());
            match self.variant {
                LevelEntityVariant::Deletion => {
                    println!("Not yet implemented: Deletion");
                }
                LevelEntityVariant::Background => {
                    level.background.push(entity);
                }
                LevelEntityVariant::Indestructible => {
                    level.indestructible.push(entity);
                }
                LevelEntityVariant::Destructible => {
                    level.destructible.push(entity);
                }
                LevelEntityVariant::Enemies => {
                    level.enemies.push(entity);
                }
                LevelEntityVariant::MainCharacter => {
                    level.main_character.push(entity);
                }
                LevelEntityVariant::Effects => {
                    level.effects.push(entity);
                }
                LevelEntityVariant::Foreground => {
                    level.foreground.push(entity);
                }
            }
        }
    }
}

pub mod level {
    use super::camera::camera::Camera;
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
                acceleration_y: 1.0,
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
            let intended_velocity = (
                self.velocity_x + self.acceleration_x,
                self.velocity_y + self.acceleration_y,
            );
            let intended_position = (
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
        pub fn is_inside_bounds(&self, position: (i32, i32), width: u16, height: u16) -> bool {
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

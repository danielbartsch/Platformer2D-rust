#[path = "camera.rs"]
mod camera;

#[path = "level.rs"]
mod level;

#[path = "editor_menu.rs"]
mod editor_menu;

use camera::Camera;
use editor_menu::EditorMenu;
use level::{Entity, EntityVariant, Level};
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
                let x = (entity.x * $camera.get_scale_x()) as i32 + (WINDOW_WIDTH as i32 / 2)
                    - ($camera.get_x() as f32 * (entity.parallax_x * $camera.get_scale_x())) as i32;
                let y = (entity.y * $camera.get_scale_y()) as i32 + (WINDOW_HEIGHT as i32 / 2)
                    - ($camera.get_y() as f32 * (entity.parallax_y * $camera.get_scale_y())) as i32;

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

pub fn run(level_name: &str) {
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

    let mut level1 = Level::deserialize(
        fs::read_to_string(format!("assets/levels/{}.json", level_name)).unwrap(),
    );

    let mut camera = Camera::new(900, 600);
    let mut target_camera = Camera::new(900, 600);

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
                            target_camera.zoom(0.97);
                        } else {
                            target_camera.zoom(1.03);
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
                                        mouse_selection_rect.unwrap().x()
                                            + (camera.position.0 as i32)
                                            - (WINDOW_WIDTH / 2) as i32,
                                        mouse_selection_rect.unwrap().y()
                                            + (camera.position.1 as i32)
                                            - (WINDOW_HEIGHT / 2) as i32,
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
                target_camera.position.1 -= 25.0 / target_camera.scale.1;
            } else if pressed_keys.contains(&Keycode::S) {
                target_camera.position.1 += 25.0 / target_camera.scale.1;
            }
            if pressed_keys.contains(&Keycode::A) {
                target_camera.position.0 -= 25.0 / target_camera.scale.0;
            } else if pressed_keys.contains(&Keycode::H) {
                target_camera.position.0 += 25.0 / target_camera.scale.1;
            }
            if pressed_keys.contains(&Keycode::Q) {
                target_camera.zoom(1.03);
            } else if pressed_keys.contains(&Keycode::R) {
                target_camera.zoom(0.97);
            }
        } else {
            target_camera.set_zoom(1.0);
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
                target_camera.position.1 = level1.main_character[character_index].y - 400.0;
            } else if pressed_keys.contains(&Keycode::S) {
                target_camera.position.1 = level1.main_character[character_index].y + 400.0;
                level1.main_character[character_index].velocity_y = 5.0;
            }
            if pressed_keys.contains(&Keycode::A) {
                target_camera.position.0 = level1.main_character[character_index].x - 400.0;
                level1.main_character[character_index].velocity_x = -5.0;
            } else if pressed_keys.contains(&Keycode::H) {
                target_camera.position.0 = level1.main_character[character_index].x + 400.0;
                level1.main_character[character_index].velocity_x = 5.0;
            } else {
                level1.main_character[character_index].velocity_x *= 0.8;
            }
            if !pressed_keys.contains(&Keycode::A)
                && !pressed_keys.contains(&Keycode::S)
                && !pressed_keys.contains(&Keycode::H)
                && !pressed_keys.contains(&Keycode::D)
            {
                target_camera.position = (
                    level1.main_character[character_index].x,
                    level1.main_character[character_index].y,
                );
            }
        }

        for character in &mut level1.main_character {
            character.next_state(entities.clone());
        }
        for character in &mut level1.effects {
            character.next_state(entities.clone());
        }

        camera.to_target(
            &target_camera,
            if edit_mode { (0.3, 0.3) } else { (0.03, 0.01) },
        );

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
                if mem::discriminant(&variant) == mem::discriminant(&editor_menu.entity_variant) {
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

        let millis_to_sleep =
            MAX_FRAME_TIME_MILLIS as i32 - last_frame_time.elapsed().unwrap().as_millis() as i32;
        if millis_to_sleep > 0 {
            ::std::thread::sleep(Duration::new(0, millis_to_sleep as u32 * 1_000_000u32));
        } else {
            println!("Detecting Lag. Frame took {}ms too long", -millis_to_sleep);
        }
        last_frame_time = SystemTime::now();
    }
}

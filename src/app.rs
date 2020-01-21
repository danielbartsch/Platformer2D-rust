#[path = "camera.rs"]
mod camera;

#[path = "level.rs"]
mod level;

#[path = "editor_menu.rs"]
mod editor_menu;

use camera::Camera;
use editor_menu::EditorMenu;
use level::{Entity, Level};
use sdl2::event::{Event, WindowEvent};
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

static MAX_FRAME_TIME_MILLIS: u64 = 16;

macro_rules! draw_relatively {
    ($canvas: expr, $entities: expr, $camera: expr, $texture: expr, $dimensions: expr) => {
        if $entities.len() > 0 {
            for entity in $entities {
                let (_x, _y, _width, _height) =
                    entity.to_canvas_coordinates($camera, ($dimensions.0 / 2, $dimensions.1 / 2));
                let (x, y, width, height) = (_x as i32, _y as i32, _width as u32, _height as u32);

                if x + width as i32 >= 0
                    && y + height as i32 >= 0
                    && x <= $dimensions.0 as i32
                    && y <= $dimensions.1 as i32
                {
                    let entity_rect = Rect::new(x, y, width, height);
                    if let Some(sprite_rect) = entity.sprite_sheet_rect {
                        $canvas
                            .copy_ex(
                                $texture,
                                Some(Rect::new(
                                    sprite_rect.0,
                                    sprite_rect.1,
                                    sprite_rect.2,
                                    sprite_rect.3,
                                )),
                                Some(entity_rect),
                                0.0,
                                None,
                                false,
                                false,
                            )
                            .unwrap();
                    } else {
                        let original_color = $canvas.draw_color();
                        $canvas.set_draw_color(LINE_BACKGROUND_COLOR);
                        $canvas.fill_rect(entity_rect).unwrap();
                        $canvas.set_draw_color(LINE_COLOR);
                        $canvas.draw_rect(entity_rect).unwrap();
                        $canvas.set_draw_color(original_color);
                    }
                }
            }
        }
    };
}

pub fn run(level_name: &str, sprite_sheet_name: &str) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window_width: u32 = 900;
    let mut window_height: u32 = 600;

    let window = video_subsystem
        .window("Platformer 2D", window_width, window_height)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();
    canvas.window_mut().set_minimum_size(350, 250).unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    video_subsystem.text_input().start();

    let mut level = Level::deserialize(
        fs::read_to_string(format!("assets/levels/{}.json", level_name)).unwrap(),
    );

    let mut temp_surface = sdl2::surface::Surface::load_bmp(std::path::Path::new(&format!(
        "assets/spritesheets/{}.bmp",
        sprite_sheet_name
    )))
    .unwrap();
    temp_surface
        .set_color_key(true, sdl2::pixels::Color::RGB(0, 0, 0))
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&temp_surface)
        .unwrap();

    let mut paused = false;

    let mut camera = Camera::new(900, 600);
    let mut target_camera = Camera::new(900, 600);

    let mut character_index = 0;

    let first_frame_time = SystemTime::now();
    let mut last_frame_time = first_frame_time.clone();

    let mut edit_mode = false;
    let mut editor_menu = EditorMenu::new();

    let mut mouse_click_position = None;
    let mut mouse_selection_rect: Option<Rect> = None;

    'running: loop {
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();

        let ticks = first_frame_time.elapsed().unwrap().as_millis();

        let mut pressed_keys = HashSet::new();
        pressed_keys = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mouse_state = event_pump.mouse_state();

        let (mouse_x, mouse_y) = (mouse_state.x(), mouse_state.y());

        let has_free_camera = edit_mode || paused;

        let mut camera_commands: Vec<Box<dyn FnMut(&mut Entity, &mut Camera)>> = vec![];

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => {
                        if edit_mode {
                            mouse_click_position = None;
                        }
                        paused = !paused;
                    }
                    Some(Keycode::P) => {
                        character_index = (character_index + 1) % level.main_character.len();
                    }
                    Some(Keycode::Num0) => {
                        edit_mode = !edit_mode;
                    }
                    _ => {}
                },
                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => {
                    window_width = width as u32;
                    window_height = height as u32;
                }
                Event::MouseWheel { y, .. } => {
                    if has_free_camera {
                        if y < 0 {
                            camera_commands.push(Box::new(|entity, current_camera| {
                                current_camera.zoom(0.97);
                            }));
                        } else {
                            camera_commands.push(Box::new(|entity, current_camera| {
                                current_camera.zoom(1.03);
                            }));
                        }
                    }
                }
                Event::MouseButtonDown { x, y, .. } => {
                    if has_free_camera {
                        let clicked_variant_button = EditorMenu::get_variant_button_rects()
                            .into_iter()
                            .find(|(_, rect)| {
                                x > rect.x()
                                    && x < rect.x() + rect.width() as i32
                                    && y > rect.y()
                                    && y < rect.y() + rect.height() as i32
                            });
                        if mem::discriminant(&clicked_variant_button) == mem::discriminant(&None) {
                            match mouse_click_position {
                                Some(_) => {
                                    mouse_click_position = None;

                                    editor_menu.create_entity(
                                        &mut level,
                                        &Entity::from_canvas_coordinates(
                                            (
                                                mouse_selection_rect.unwrap().x() as f32,
                                                mouse_selection_rect.unwrap().y() as f32,
                                                mouse_selection_rect.unwrap().width() as u16,
                                                mouse_selection_rect.unwrap().height() as u16,
                                                1.0,
                                                1.0,
                                            ),
                                            &camera,
                                            (window_width / 2, window_height / 2),
                                        ),
                                    );
                                    mouse_selection_rect = None;
                                }
                                None => {
                                    mouse_click_position = Some((x, y));
                                }
                            }
                        } else if let Some((variant, _)) = clicked_variant_button {
                            editor_menu.variant(variant);
                        }
                    }
                }
                _ => {}
            }
        }

        let mut entities = vec![];
        entities.extend(&level.indestructible);
        entities.extend(&level.destructible);
        entities.extend(&level.enemies);

        if has_free_camera {
            if pressed_keys.contains(&Keycode::D) {
                camera_commands.push(Box::new(|entity, current_camera| {
                    current_camera.position.1 -= 25.0 / current_camera.scale.1;
                }));
            } else if pressed_keys.contains(&Keycode::S) {
                camera_commands.push(Box::new(|entity, current_camera| {
                    current_camera.position.1 += 25.0 / current_camera.scale.1;
                }));
            }
            if pressed_keys.contains(&Keycode::A) {
                camera_commands.push(Box::new(|entity, current_camera| {
                    current_camera.position.0 -= 25.0 / current_camera.scale.0;
                }));
            } else if pressed_keys.contains(&Keycode::H) {
                camera_commands.push(Box::new(|entity, current_camera| {
                    current_camera.position.0 += 25.0 / current_camera.scale.1;
                }));
            }
            if pressed_keys.contains(&Keycode::Q) {
                camera_commands.push(Box::new(|entity, current_camera| {
                    current_camera.zoom(1.03);
                }));
            } else if pressed_keys.contains(&Keycode::R) {
                camera_commands.push(Box::new(|entity, current_camera| {
                    current_camera.zoom(0.97);
                }));
            }
        } else {
            camera_commands.push(Box::new(|entity, current_camera| {
                current_camera.set_zoom(1.0);
            }));

            let mut entity_commands: Vec<Box<dyn FnMut(&mut Entity)>> = vec![];
            let mut attack_commands: Vec<Box<dyn FnMut(&mut Entity, &mut Vec<Entity>)>> = vec![];

            if pressed_keys.contains(&Keycode::Y) {
                attack_commands.push(Box::new(|entity, level_container| {
                    let pseudo_random = last_frame_time.elapsed().unwrap().as_nanos() as f32;
                    level_container.push(
                        Entity::new(entity.x, entity.y, 10, 10)
                            .velocity_x(entity.velocity_x * (2.2 + pseudo_random.cos()))
                            .velocity_y(entity.velocity_y * (2.2 + pseudo_random.sin()))
                            .acceleration_y(0.01)
                            .bounciness(1.1),
                    );
                }));
            }
            if pressed_keys.contains(&Keycode::N) {
                entity_commands.push(Box::new(|entity| {
                    if entity.is_touching_ground(entities.clone()) {
                        entity.velocity_y = -8.0;
                        entity.acceleration_y = 0.1;
                    }
                    if entity.velocity_y < 0.0 {
                        entity.acceleration_y += 0.01;
                    } else {
                        entity.acceleration_y += 0.002;
                    }
                }));
            } else {
                entity_commands.push(Box::new(|entity| {
                    entity.acceleration_y = 1.0;
                }));
            }
            if pressed_keys.contains(&Keycode::D) {
                camera_commands.push(Box::new(|entity, current_camera| {
                    current_camera.position.1 = entity.y - 400.0;
                }));
            } else if pressed_keys.contains(&Keycode::S) {
                camera_commands.push(Box::new(|entity, current_camera| {
                    current_camera.position.1 = entity.y + 400.0;
                }));
            }
            if pressed_keys.contains(&Keycode::A) {
                camera_commands.push(Box::new(|entity, current_camera| {
                    current_camera.position.0 = entity.x - 400.0;
                }));
                entity_commands.push(Box::new(|entity| {
                    entity.velocity_x = -5.0;
                    if ticks % 300 > 150 {
                        entity.sprite_sheet_rect = Some((0, 0, 32, 32));
                    } else {
                        entity.sprite_sheet_rect = Some((32, 0, 32, 32));
                    }
                }));
            } else if pressed_keys.contains(&Keycode::H) {
                camera_commands.push(Box::new(|entity, current_camera| {
                    current_camera.position.0 = entity.x + 400.0;
                }));
                entity_commands.push(Box::new(|entity| {
                    entity.velocity_x = 5.0;
                    if ticks % 300 > 150 {
                        entity.sprite_sheet_rect = Some((64, 0, 32, 32));
                    } else {
                        entity.sprite_sheet_rect = Some((96, 0, 32, 32));
                    }
                }));
            } else {
                entity_commands.push(Box::new(|entity| {
                    entity.velocity_x *= 0.8;
                }));
            }
            if !pressed_keys.contains(&Keycode::A)
                && !pressed_keys.contains(&Keycode::S)
                && !pressed_keys.contains(&Keycode::H)
                && !pressed_keys.contains(&Keycode::D)
            {
                camera_commands.push(Box::new(|entity, current_camera| {
                    current_camera.position = (
                        entity.x + entity.width as f32 / 2.0,
                        entity.y + entity.height as f32 / 2.0,
                    );
                }));
            }

            for mut command in entity_commands {
                command(&mut level.main_character[character_index]);
            }
            for mut command in attack_commands {
                command(
                    &mut level.main_character[character_index],
                    &mut level.effects,
                );
            }
        }
        for mut command in camera_commands {
            command(
                &mut level.main_character[character_index],
                &mut target_camera,
            );
        }

        if !paused {
            for character in &mut level.main_character {
                character.next_state(entities.clone());
            }
            for character in &mut level.effects {
                character.next_state(entities.clone());
            }
        }

        camera.to_target(
            &target_camera,
            if has_free_camera {
                (0.3, 0.3)
            } else {
                (0.03, 0.03)
            },
        );

        let dimensions = (window_width, window_height);

        draw_relatively!(canvas, &level.background, &camera, &texture, dimensions);
        draw_relatively!(canvas, &level.indestructible, &camera, &texture, dimensions);
        draw_relatively!(canvas, &level.destructible, &camera, &texture, dimensions);
        draw_relatively!(canvas, &level.enemies, &camera, &texture, dimensions);
        draw_relatively!(canvas, &level.main_character, &camera, &texture, dimensions);
        draw_relatively!(canvas, &level.effects, &camera, &texture, dimensions);
        draw_relatively!(canvas, &level.foreground, &camera, &texture, dimensions);

        if paused {
            let original_color = canvas.draw_color();
            canvas.set_draw_color(Color {
                r: 255,
                g: 60,
                b: 60,
                a: 0xff,
            });

            let stop_width = 8u32;
            let stop_height = 20u32;
            let bar_gap = 4i32;
            let stop_1_bar_x = 30i32;
            let stop_1_bar_y = 0i32;

            canvas
                .fill_rect(Rect::new(
                    stop_1_bar_x,
                    stop_1_bar_y,
                    stop_width,
                    stop_height,
                ))
                .unwrap();
            canvas
                .fill_rect(Rect::new(
                    stop_1_bar_x + stop_width as i32 + bar_gap,
                    stop_1_bar_y,
                    stop_width,
                    stop_height,
                ))
                .unwrap();
            canvas.set_draw_color(original_color);
        }

        if edit_mode {
            let original_color = canvas.draw_color();
            canvas.set_draw_color(Color {
                r: 255,
                g: 60,
                b: 60,
                a: 0xff,
            });

            // Crosshair to indicate center of frame
            canvas
                .draw_line(
                    (window_width as i32 / 2, 0),
                    (window_width as i32 / 2, window_height as i32),
                )
                .unwrap();
            canvas
                .draw_line(
                    (0, window_height as i32 / 2),
                    (window_width as i32, window_height as i32 / 2),
                )
                .unwrap();

            // Edit mode "logo"
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

            match mouse_click_position {
                Some((x, y)) => {
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
                }
                None => {}
            }
            canvas.set_draw_color(original_color);
        }

        canvas.present();

        let millis_to_sleep =
            MAX_FRAME_TIME_MILLIS - last_frame_time.elapsed().unwrap().as_millis() as u64;
        if millis_to_sleep > 0 {
            ::std::thread::sleep(Duration::from_millis(millis_to_sleep));
        } else {
            println!(
                "Detecting Lag. Frame took {}ms too long",
                -(millis_to_sleep as i32)
            );
        }
        last_frame_time = SystemTime::now();
    }
}

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
use sdl2::render::{Texture, WindowCanvas};
use sdl2::surface::Surface;
use std::cmp;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::mem;
use std::path::Path;
use std::time::{Duration, SystemTime};

static BACKGROUND_COLOR: Color = Color { r: 42, g: 43, b: 37, a: 0xff };

static MAX_FRAME_TIME_MILLIS: u64 = 16;

static INITIAL_WINDOW_WIDTH: u16 = 900;
static INITIAL_WINDOW_HEIGHT: u16 = 600;

pub fn run(level_name: &str, sprite_sheet_name: &str) {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let window = video_subsystem
    .window("Platformer 2D", INITIAL_WINDOW_WIDTH as u32, INITIAL_WINDOW_HEIGHT as u32)
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

  let mut level =
    Level::deserialize(fs::read_to_string(format!("assets/levels/{}.json", level_name)).unwrap());

  let texture_creator = canvas.texture_creator();
  let (entity_texture, ui_texture, text_texture) = {
    let mut texture_surface =
      Surface::load_bmp(Path::new(&format!("assets/spritesheets/{}.bmp", sprite_sheet_name)))
        .unwrap();
    texture_surface.set_color_key(true, Color { r: 0, g: 0, b: 0, a: 0xff }).unwrap();

    let mut ui_texture_surface =
      Surface::load_bmp(Path::new("assets/spritesheets/ui.bmp")).unwrap();
    ui_texture_surface.set_color_key(true, Color { r: 0, g: 0, b: 0, a: 0xff }).unwrap();

    let mut text_texture_surface =
      Surface::load_bmp(Path::new("assets/spritesheets/text.bmp")).unwrap();
    text_texture_surface.set_color_key(true, Color { r: 255, g: 255, b: 255, a: 0xff }).unwrap();

    (
      texture_creator.create_texture_from_surface(&texture_surface).unwrap(),
      texture_creator.create_texture_from_surface(&ui_texture_surface).unwrap(),
      texture_creator.create_texture_from_surface(&text_texture_surface).unwrap(),
    )
  };

  let mut paused = false;

  let mut camera = Camera::new((INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT));
  let mut target_camera = Camera::new((INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT));

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

    let pressed_keys: HashSet<_> =
      event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

    let (mouse_x, mouse_y) = {
      let mouse_state = event_pump.mouse_state();
      (mouse_state.x(), mouse_state.y())
    };

    let has_free_camera = edit_mode || paused;

    let mut camera_commands: Vec<Box<dyn Fn(&mut Entity, &mut Camera)>> = vec![];

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
          Some(Keycode::S) => {
            if pressed_keys.contains(&Keycode::LCtrl) || pressed_keys.contains(&Keycode::RCtrl) {
              let file_path = {
                let file_name =
                  format!("{:x}.json", first_frame_time.elapsed().unwrap().as_nanos());
                format!("assets/levels/{}", file_name)
              };
              let path = Path::new(&file_path);
              let display = path.display();
              match File::create(&path) {
                Err(_) => panic!("couldn't create {}", display),
                Ok(mut file) => {
                  let level_serialized = level.serialize();
                  println!("Saving now: {}", file_path);
                  file
                    .write_all(level_serialized.as_bytes())
                    .expect(&format!("Save for file '{}' failed.", file_path))
                }
              };
            }
          }
          _ => {}
        },
        Event::Window { win_event: WindowEvent::Resized(width, height), .. } => {
          camera.dimensions = (width as u16, height as u16);
        }
        Event::MouseWheel { y, .. } => {
          if has_free_camera {
            if y < 0 {
              camera_commands.push(Box::new(|_, current_camera| {
                current_camera.zoom(0.97);
              }));
            } else {
              camera_commands.push(Box::new(|_, current_camera| {
                current_camera.zoom(1.03);
              }));
            }
          }
        }
        Event::MouseButtonDown { x, y, .. } => {
          if has_free_camera {
            let clicked_variant_button =
              EditorMenu::get_variant_button_rects().into_iter().find(|(_, rect, _)| {
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
                      (camera.dimensions.0 / 2, camera.dimensions.1 / 2),
                    ),
                  );
                  mouse_selection_rect = None;
                }
                None => {
                  mouse_click_position = Some((x, y));
                }
              }
            } else if let Some((variant, _, _)) = clicked_variant_button {
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
        camera_commands.push(Box::new(|_, current_camera| {
          current_camera.position.1 -= 25.0 / current_camera.scale.1;
        }));
      } else if pressed_keys.contains(&Keycode::S) {
        camera_commands.push(Box::new(|_, current_camera| {
          current_camera.position.1 += 25.0 / current_camera.scale.1;
        }));
      }
      if pressed_keys.contains(&Keycode::A) {
        camera_commands.push(Box::new(|_, current_camera| {
          current_camera.position.0 -= 25.0 / current_camera.scale.0;
        }));
      } else if pressed_keys.contains(&Keycode::H) {
        camera_commands.push(Box::new(|_, current_camera| {
          current_camera.position.0 += 25.0 / current_camera.scale.1;
        }));
      }
      if pressed_keys.contains(&Keycode::Q) {
        camera_commands.push(Box::new(|_, current_camera| {
          current_camera.zoom(1.03);
        }));
      } else if pressed_keys.contains(&Keycode::R) {
        camera_commands.push(Box::new(|_, current_camera| {
          current_camera.zoom(0.97);
        }));
      }
    } else {
      camera_commands.push(Box::new(|_, current_camera| {
        current_camera.set_zoom(1.0);
      }));

      let mut entity_commands: Vec<Box<dyn Fn(&mut Entity)>> = vec![];
      let mut attack_commands: Vec<Box<dyn Fn(&mut Entity, &mut Vec<Entity>)>> = vec![];

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
      let sprint_factor = if pressed_keys.contains(&Keycode::Space) { 2.0 } else { 1.0 };
      if pressed_keys.contains(&Keycode::A) {
        camera_commands.push(Box::new(|entity, current_camera| {
          current_camera.position.0 = entity.x - 400.0;
        }));
        entity_commands.push(Box::new(|entity| {
          entity.velocity_x = -5.0 * sprint_factor;
          if ticks % (300.0 / sprint_factor) as u128 > (225.0 / sprint_factor) as u128 {
            entity.sprite_sheet_rect = Some((0, 0, 32, 32));
          } else if ticks % (300.0 / sprint_factor) as u128 > (150.0 / sprint_factor) as u128 {
            entity.sprite_sheet_rect = Some((32, 0, 32, 32));
          } else if ticks % (300.0 / sprint_factor) as u128 > (75.0 / sprint_factor) as u128 {
            entity.sprite_sheet_rect = Some((64, 0, 32, 32));
          } else {
            entity.sprite_sheet_rect = Some((32, 0, 32, 32));
          }
        }));
      } else if pressed_keys.contains(&Keycode::H) {
        camera_commands.push(Box::new(|entity, current_camera| {
          current_camera.position.0 = entity.x + 400.0;
        }));
        entity_commands.push(Box::new(|entity| {
          entity.velocity_x = 5.0 * sprint_factor;
          if ticks % (300.0 / sprint_factor) as u128 > (225.0 / sprint_factor) as u128 {
            entity.sprite_sheet_rect = Some((96, 0, 32, 32));
          } else if ticks % (300.0 / sprint_factor) as u128 > (150.0 / sprint_factor) as u128 {
            entity.sprite_sheet_rect = Some((128, 0, 32, 32));
          } else if ticks % (300.0 / sprint_factor) as u128 > (75.0 / sprint_factor) as u128 {
            entity.sprite_sheet_rect = Some((160, 0, 32, 32));
          } else {
            entity.sprite_sheet_rect = Some((128, 0, 32, 32));
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
          current_camera.position =
            (entity.x + entity.width as f32 / 2.0, entity.y + entity.height as f32 / 2.0);
        }));
      }

      for command in entity_commands {
        command(&mut level.main_character[character_index]);
      }
      for command in attack_commands {
        command(&mut level.main_character[character_index], &mut level.effects);
      }
    }
    for command in camera_commands {
      command(&mut level.main_character[character_index], &mut target_camera);
    }

    if !paused {
      for character in &mut level.main_character {
        character.next_state(entities.clone());
      }
      for character in &mut level.effects {
        character.next_state(entities.clone());
      }
    }

    camera.to_target(&target_camera, if has_free_camera { (0.3, 0.3) } else { (0.03, 0.03) });

    camera.draw_relatively(&mut canvas, &level.background, &entity_texture);
    camera.draw_relatively(&mut canvas, &level.indestructible, &entity_texture);
    camera.draw_relatively(&mut canvas, &level.destructible, &entity_texture);
    camera.draw_relatively(&mut canvas, &level.enemies, &entity_texture);
    camera.draw_relatively(&mut canvas, &level.main_character, &entity_texture);
    camera.draw_relatively(&mut canvas, &level.effects, &entity_texture);
    camera.draw_relatively(&mut canvas, &level.foreground, &entity_texture);

    if paused {
      let original_color = canvas.draw_color();
      canvas.set_draw_color(Color { r: 255, g: 60, b: 60, a: 0xff });

      let stop_width = 8u32;
      let stop_height = 20u32;
      let bar_gap = 4i32;
      let stop_1_bar_x = 30i32;
      let stop_1_bar_y = 0i32;

      canvas.fill_rect(Rect::new(stop_1_bar_x, stop_1_bar_y, stop_width, stop_height)).unwrap();
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
      canvas.set_draw_color(Color { r: 255, g: 60, b: 60, a: 0xff });

      // Crosshair to indicate center of frame
      canvas
        .draw_line(
          (camera.dimensions.0 as i32 / 2, 0),
          (camera.dimensions.0 as i32 / 2, camera.dimensions.1 as i32),
        )
        .unwrap();
      canvas
        .draw_line(
          (0, camera.dimensions.1 as i32 / 2),
          (camera.dimensions.0 as i32, camera.dimensions.1 as i32 / 2),
        )
        .unwrap();

      // Edit mode "logo"
      canvas
        .copy_ex(
          &ui_texture,
          Some(Rect::new(0, 0, 20, 20)),
          Some(Rect::new(0, 0, 20, 20)),
          0.0,
          None,
          false,
          false,
        )
        .unwrap();

      for (variant, rect, sprite_rect) in EditorMenu::get_variant_button_rects() {
        canvas
          .copy_ex(
            &ui_texture,
            Some(Rect::new(sprite_rect.0, sprite_rect.1, sprite_rect.2, sprite_rect.3)),
            Some(rect),
            0.0,
            None,
            false,
            false,
          )
          .unwrap();
        if mem::discriminant(&variant) == mem::discriminant(&editor_menu.variant) {
          canvas.draw_rect(rect).unwrap();
        }
      }

      match mouse_click_position {
        Some((x, y)) => {
          canvas.set_draw_color(Color { r: 255, g: 60, b: 60, a: 0xff });
          let (pos_x, width) = (cmp::min(x, mouse_x), (x - mouse_x).wrapping_abs() as u32);
          let (pos_y, height) = (cmp::min(y, mouse_y), (y - mouse_y).wrapping_abs() as u32);
          mouse_selection_rect = Some(Rect::new(pos_x, pos_y, width, height));
          canvas.draw_rect(mouse_selection_rect.unwrap()).unwrap();
        }
        None => {}
      }
      canvas.set_draw_color(original_color);
    }

    show_text_line(
      &mut canvas,
      &text_texture,
      &format!(
        "ENTITIES: {}",
        level.background.len()
          + level.indestructible.len()
          + level.destructible.len()
          + level.enemies.len()
          + level.main_character.len()
          + level.effects.len()
          + level.foreground.len()
      ),
      (10, 10),
      5,
      1.1,
    );
    show_text_line(
      &mut canvas,
      &text_texture,
      &format!("FRAME TIME MICROSECONDS: {}", last_frame_time.elapsed().unwrap().as_micros()),
      (10, 100),
      2,
      1.1,
    );
    show_text_line(
      &mut canvas,
      &text_texture,
      &format!("CAMERA SCALE: {}", camera.scale.0),
      (10, 210),
      2,
      1.1,
    );

    canvas.present();

    let millis_to_sleep =
      MAX_FRAME_TIME_MILLIS - last_frame_time.elapsed().unwrap().as_millis() as u64;
    if millis_to_sleep > 0 {
      ::std::thread::sleep(Duration::from_millis(millis_to_sleep));
    } else {
      println!("Detecting Lag. Frame took {}ms too long", -(millis_to_sleep as i32));
    }
    last_frame_time = SystemTime::now();
  }
}

fn show_text_line(
  canvas: &mut WindowCanvas,
  texture: &Texture,
  text: &str,
  position: (i32, i32),
  letter_scale: u8,
  letter_gap: f32,
) {
  let chars = text.chars().enumerate();
  for (index, character) in chars {
    let rekt = get_text_texture_rects(character);

    let letter_width = rekt.width() * letter_scale as u32;
    let letter_height = rekt.height() * letter_scale as u32;

    canvas
      .copy_ex(
        &texture,
        Some(rekt),
        Some(Rect::new(
          position.0 + (index as f32 * letter_width as f32 * letter_gap) as i32,
          position.1,
          letter_width as u32,
          letter_height as u32,
        )),
        0.0,
        None,
        false,
        false,
      )
      .unwrap();
  }
}

fn get_text_texture_rects(character: char) -> Rect {
  let char_width = 7;
  let char_height = 9;
  match character {
    'A' => Rect::new(char_width as i32 * 0, 0, char_width, char_height),
    'a' => Rect::new(char_width as i32 * 0, char_height as i32, char_width, char_height),
    'B' => Rect::new(char_width as i32 * 1, 0, char_width, char_height),
    'b' => Rect::new(char_width as i32 * 1, char_height as i32, char_width, char_height),
    'C' => Rect::new(char_width as i32 * 2, 0, char_width, char_height),
    'c' => Rect::new(char_width as i32 * 2, char_height as i32, char_width, char_height),
    'D' => Rect::new(char_width as i32 * 3, 0, char_width, char_height),
    'd' => Rect::new(char_width as i32 * 3, char_height as i32, char_width, char_height),
    'E' => Rect::new(char_width as i32 * 4, 0, char_width, char_height),
    'e' => Rect::new(char_width as i32 * 4, char_height as i32, char_width, char_height),
    'F' => Rect::new(char_width as i32 * 5, 0, char_width, char_height),
    'f' => Rect::new(char_width as i32 * 5, char_height as i32, char_width, char_height),
    'G' => Rect::new(char_width as i32 * 6, 0, char_width, char_height),
    'g' => Rect::new(char_width as i32 * 6, char_height as i32, char_width, char_height),
    'H' => Rect::new(char_width as i32 * 7, 0, char_width, char_height),
    'h' => Rect::new(char_width as i32 * 7, char_height as i32, char_width, char_height),
    'I' => Rect::new(char_width as i32 * 8, 0, char_width, char_height),
    'i' => Rect::new(char_width as i32 * 8, char_height as i32, char_width, char_height),
    'J' => Rect::new(char_width as i32 * 9, 0, char_width, char_height),
    'j' => Rect::new(char_width as i32 * 9, char_height as i32, char_width, char_height),
    'K' => Rect::new(char_width as i32 * 10, 0, char_width, char_height),
    'k' => Rect::new(char_width as i32 * 10, char_height as i32, char_width, char_height),
    'L' => Rect::new(char_width as i32 * 11, 0, char_width, char_height),
    'l' => Rect::new(char_width as i32 * 11, char_height as i32, char_width, char_height),
    'M' => Rect::new(char_width as i32 * 12, 0, char_width, char_height),
    'm' => Rect::new(char_width as i32 * 12, char_height as i32, char_width, char_height),
    'N' => Rect::new(char_width as i32 * 13, 0, char_width, char_height),
    'n' => Rect::new(char_width as i32 * 13, char_height as i32, char_width, char_height),
    'O' => Rect::new(char_width as i32 * 14, 0, char_width, char_height),
    'o' => Rect::new(char_width as i32 * 14, char_height as i32, char_width, char_height),
    'P' => Rect::new(char_width as i32 * 15, 0, char_width, char_height),
    'p' => Rect::new(char_width as i32 * 15, char_height as i32, char_width, char_height),
    'Q' => Rect::new(char_width as i32 * 16, 0, char_width, char_height),
    'q' => Rect::new(char_width as i32 * 16, char_height as i32, char_width, char_height),
    'R' => Rect::new(char_width as i32 * 17, 0, char_width, char_height),
    'r' => Rect::new(char_width as i32 * 17, char_height as i32, char_width, char_height),
    'S' => Rect::new(char_width as i32 * 18, 0, char_width, char_height),
    's' => Rect::new(char_width as i32 * 18, char_height as i32, char_width, char_height),
    'T' => Rect::new(char_width as i32 * 19, 0, char_width, char_height),
    't' => Rect::new(char_width as i32 * 19, char_height as i32, char_width, char_height),
    'U' => Rect::new(char_width as i32 * 20, 0, char_width, char_height),
    'u' => Rect::new(char_width as i32 * 20, char_height as i32, char_width, char_height),
    'V' => Rect::new(char_width as i32 * 21, 0, char_width, char_height),
    'v' => Rect::new(char_width as i32 * 21, char_height as i32, char_width, char_height),
    'W' => Rect::new(char_width as i32 * 22, 0, char_width, char_height),
    'w' => Rect::new(char_width as i32 * 22, char_height as i32, char_width, char_height),
    'X' => Rect::new(char_width as i32 * 23, 0, char_width, char_height),
    'x' => Rect::new(char_width as i32 * 23, char_height as i32, char_width, char_height),
    'Y' => Rect::new(char_width as i32 * 24, 0, char_width, char_height),
    'y' => Rect::new(char_width as i32 * 24, char_height as i32, char_width, char_height),
    'Z' => Rect::new(char_width as i32 * 25, 0, char_width, char_height),
    'z' => Rect::new(char_width as i32 * 25, char_height as i32, char_width, char_height),
    '0' => Rect::new(char_width as i32 * 26, 0, char_width, char_height),
    '1' => Rect::new(char_width as i32 * 27, 0, char_width, char_height),
    '2' => Rect::new(char_width as i32 * 28, 0, char_width, char_height),
    '3' => Rect::new(char_width as i32 * 29, 0, char_width, char_height),
    '4' => Rect::new(char_width as i32 * 30, 0, char_width, char_height),
    '5' => Rect::new(char_width as i32 * 31, 0, char_width, char_height),
    '6' => Rect::new(char_width as i32 * 32, 0, char_width, char_height),
    '7' => Rect::new(char_width as i32 * 33, 0, char_width, char_height),
    '8' => Rect::new(char_width as i32 * 34, 0, char_width, char_height),
    '9' => Rect::new(char_width as i32 * 35, 0, char_width, char_height),
    ',' => Rect::new(char_width as i32 * 36, 0, char_width, char_height),
    '.' => Rect::new(char_width as i32 * 37, 0, char_width, char_height),
    '!' => Rect::new(char_width as i32 * 38, 0, char_width, char_height),
    '?' => Rect::new(char_width as i32 * 39, 0, char_width, char_height),
    ':' => Rect::new(char_width as i32 * 40, 0, char_width, char_height),
    ';' => Rect::new(char_width as i32 * 41, 0, char_width, char_height),
    '"' => Rect::new(char_width as i32 * 42, 0, char_width, char_height),
    '{' => Rect::new(char_width as i32 * 43, 0, char_width, char_height),
    '}' => Rect::new(char_width as i32 * 44, 0, char_width, char_height),
    '[' => Rect::new(char_width as i32 * 45, 0, char_width, char_height),
    ']' => Rect::new(char_width as i32 * 46, 0, char_width, char_height),
    '(' => Rect::new(char_width as i32 * 47, 0, char_width, char_height),
    ')' => Rect::new(char_width as i32 * 48, 0, char_width, char_height),
    ' ' => Rect::new(0, 0, 0, 0),
    _ => Rect::new(5000, 0, char_width, char_height),
  }
}

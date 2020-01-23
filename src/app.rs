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
  let (entity_texture, ui_texture) = {
    let mut texture_surface =
      Surface::load_bmp(Path::new(&format!("assets/spritesheets/{}.bmp", sprite_sheet_name)))
        .unwrap();
    texture_surface.set_color_key(true, Color { r: 0, g: 0, b: 0, a: 0xff }).unwrap();

    let mut ui_texture_surface =
      Surface::load_bmp(Path::new("assets/spritesheets/ui.bmp")).unwrap();
    ui_texture_surface.set_color_key(true, Color { r: 0, g: 0, b: 0, a: 0xff }).unwrap();

    (
      texture_creator.create_texture_from_surface(&texture_surface).unwrap(),
      texture_creator.create_texture_from_surface(&ui_texture_surface).unwrap(),
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

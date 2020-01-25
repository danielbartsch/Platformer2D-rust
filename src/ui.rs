use super::editor_menu::{EditorMenu, LevelEntityVariant};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use std::mem;

pub fn draw_pause_menu(canvas: &mut WindowCanvas) {
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

pub fn draw_edit_menu(
  canvas: &mut WindowCanvas,
  ui_texture: &Texture,
  editor_menu_variant: &LevelEntityVariant,
) {
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
    if mem::discriminant(&variant) == mem::discriminant(editor_menu_variant) {
      canvas.draw_rect(rect).unwrap();
    }
  }
}

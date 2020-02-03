use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

pub fn show_text_line(
  canvas: &mut WindowCanvas,
  texture: &mut Texture,
  text: &str,
  position: (i32, i32),
  letter_scale: u8,
  letter_gap: f32,
  color: Color,
) {
  let color_mod = texture.color_mod();
  texture.set_color_mod(color.r, color.g, color.b);
  let chars = text.chars().enumerate();
  for (index, character) in chars {
    let (x, y, width, height) = get_text_texture_rects(character);
    let letter_width = width * letter_scale as u32;
    let letter_height = height * letter_scale as u32;
    canvas
      .copy_ex(
        &texture,
        Some(Rect::new(x, y, width, height)),
        Some(Rect::new(
          position.0 + (index as f32 * letter_width as f32 * letter_gap) as i32,
          position.1,
          letter_width,
          letter_height,
        )),
        0.0,
        None,
        false,
        false,
      )
      .unwrap();
  }
  texture.set_color_mod(color_mod.0, color_mod.1, color_mod.2);
}

fn get_text_texture_rects(character: char) -> (i32, i32, u32, u32) {
  let char_width = 7;
  let char_height = 9;
  match character {
    'A' => (char_width as i32 * 0, 0, char_width, char_height),
    'a' => (char_width as i32 * 0, char_height as i32, char_width, char_height),
    'B' => (char_width as i32 * 1, 0, char_width, char_height),
    'b' => (char_width as i32 * 1, char_height as i32, char_width, char_height),
    'C' => (char_width as i32 * 2, 0, char_width, char_height),
    'c' => (char_width as i32 * 2, char_height as i32, char_width, char_height),
    'D' => (char_width as i32 * 3, 0, char_width, char_height),
    'd' => (char_width as i32 * 3, char_height as i32, char_width, char_height),
    'E' => (char_width as i32 * 4, 0, char_width, char_height),
    'e' => (char_width as i32 * 4, char_height as i32, char_width, char_height),
    'F' => (char_width as i32 * 5, 0, char_width, char_height),
    'f' => (char_width as i32 * 5, char_height as i32, char_width, char_height),
    'G' => (char_width as i32 * 6, 0, char_width, char_height),
    'g' => (char_width as i32 * 6, char_height as i32, char_width, char_height),
    'H' => (char_width as i32 * 7, 0, char_width, char_height),
    'h' => (char_width as i32 * 7, char_height as i32, char_width, char_height),
    'I' => (char_width as i32 * 8, 0, char_width, char_height),
    'i' => (char_width as i32 * 8, char_height as i32, char_width, char_height),
    'J' => (char_width as i32 * 9, 0, char_width, char_height),
    'j' => (char_width as i32 * 9, char_height as i32, char_width, char_height),
    'K' => (char_width as i32 * 10, 0, char_width, char_height),
    'k' => (char_width as i32 * 10, char_height as i32, char_width, char_height),
    'L' => (char_width as i32 * 11, 0, char_width, char_height),
    'l' => (char_width as i32 * 11, char_height as i32, char_width, char_height),
    'M' => (char_width as i32 * 12, 0, char_width, char_height),
    'm' => (char_width as i32 * 12, char_height as i32, char_width, char_height),
    'N' => (char_width as i32 * 13, 0, char_width, char_height),
    'n' => (char_width as i32 * 13, char_height as i32, char_width, char_height),
    'O' => (char_width as i32 * 14, 0, char_width, char_height),
    'o' => (char_width as i32 * 14, char_height as i32, char_width, char_height),
    'P' => (char_width as i32 * 15, 0, char_width, char_height),
    'p' => (char_width as i32 * 15, char_height as i32, char_width, char_height),
    'Q' => (char_width as i32 * 16, 0, char_width, char_height),
    'q' => (char_width as i32 * 16, char_height as i32, char_width, char_height),
    'R' => (char_width as i32 * 17, 0, char_width, char_height),
    'r' => (char_width as i32 * 17, char_height as i32, char_width, char_height),
    'S' => (char_width as i32 * 18, 0, char_width, char_height),
    's' => (char_width as i32 * 18, char_height as i32, char_width, char_height),
    'T' => (char_width as i32 * 19, 0, char_width, char_height),
    't' => (char_width as i32 * 19, char_height as i32, char_width, char_height),
    'U' => (char_width as i32 * 20, 0, char_width, char_height),
    'u' => (char_width as i32 * 20, char_height as i32, char_width, char_height),
    'V' => (char_width as i32 * 21, 0, char_width, char_height),
    'v' => (char_width as i32 * 21, char_height as i32, char_width, char_height),
    'W' => (char_width as i32 * 22, 0, char_width, char_height),
    'w' => (char_width as i32 * 22, char_height as i32, char_width, char_height),
    'X' => (char_width as i32 * 23, 0, char_width, char_height),
    'x' => (char_width as i32 * 23, char_height as i32, char_width, char_height),
    'Y' => (char_width as i32 * 24, 0, char_width, char_height),
    'y' => (char_width as i32 * 24, char_height as i32, char_width, char_height),
    'Z' => (char_width as i32 * 25, 0, char_width, char_height),
    'z' => (char_width as i32 * 25, char_height as i32, char_width, char_height),
    '0' => (char_width as i32 * 26, 0, char_width, char_height),
    '+' => (char_width as i32 * 26, char_height as i32, char_width, char_height),
    '1' => (char_width as i32 * 27, 0, char_width, char_height),
    '-' => (char_width as i32 * 27, char_height as i32, char_width, char_height),
    '2' => (char_width as i32 * 28, 0, char_width, char_height),
    '/' => (char_width as i32 * 28, char_height as i32, char_width, char_height),
    '3' => (char_width as i32 * 29, 0, char_width, char_height),
    '\\' => (char_width as i32 * 29, char_height as i32, char_width, char_height),
    '4' => (char_width as i32 * 30, 0, char_width, char_height),
    '5' => (char_width as i32 * 31, 0, char_width, char_height),
    '6' => (char_width as i32 * 32, 0, char_width, char_height),
    '7' => (char_width as i32 * 33, 0, char_width, char_height),
    '8' => (char_width as i32 * 34, 0, char_width, char_height),
    '9' => (char_width as i32 * 35, 0, char_width, char_height),
    ',' => (char_width as i32 * 36, 0, char_width, char_height),
    '.' => (char_width as i32 * 37, 0, char_width, char_height),
    '!' => (char_width as i32 * 38, 0, char_width, char_height),
    '?' => (char_width as i32 * 39, 0, char_width, char_height),
    ':' => (char_width as i32 * 40, 0, char_width, char_height),
    ';' => (char_width as i32 * 41, 0, char_width, char_height),
    '"' => (char_width as i32 * 42, 0, char_width, char_height),
    '{' => (char_width as i32 * 43, 0, char_width, char_height),
    '}' => (char_width as i32 * 44, 0, char_width, char_height),
    '[' => (char_width as i32 * 45, 0, char_width, char_height),
    ']' => (char_width as i32 * 46, 0, char_width, char_height),
    '(' => (char_width as i32 * 47, 0, char_width, char_height),
    ')' => (char_width as i32 * 48, 0, char_width, char_height),
    ' ' => (0, 0, 0, 0),
    _ => (5000, 0, char_width, char_height),
  }
}

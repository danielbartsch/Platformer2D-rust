use super::level::{Entity, Level};
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
}
impl EditorMenu {
  pub fn new() -> Self {
    Self { variant: LevelEntityVariant::Effects }
  }
  pub fn variant(&mut self, variant: LevelEntityVariant) {
    self.variant = variant;
  }
  pub fn get_variant_button_rects() -> Vec<(LevelEntityVariant, Rect, (i32, i32, u32, u32))> {
    vec![
      (LevelEntityVariant::Background, Rect::new(0, 30, 20, 20), (20 * 1, 0, 20, 20)),
      (LevelEntityVariant::Indestructible, Rect::new(0, 30 + (25), 20, 20), (20 * 2, 0, 20, 20)),
      (LevelEntityVariant::Destructible, Rect::new(0, 30 + (25 * 2), 20, 20), (20 * 3, 0, 20, 20)),
      (LevelEntityVariant::Enemies, Rect::new(0, 30 + (25 * 3), 20, 20), (20 * 4, 0, 20, 20)),
      (LevelEntityVariant::MainCharacter, Rect::new(0, 30 + (25 * 4), 20, 20), (20 * 5, 0, 20, 20)),
      (LevelEntityVariant::Effects, Rect::new(0, 30 + (25 * 5), 20, 20), (20 * 6, 0, 20, 20)),
      (LevelEntityVariant::Foreground, Rect::new(0, 30 + (25 * 6), 20, 20), (20 * 7, 0, 20, 20)),
      (LevelEntityVariant::Deletion, Rect::new(0, 30 + (25 * 7), 20, 20), (20 * 8, 0, 20, 20)),
    ]
  }
  pub fn create_entity(&mut self, level: &mut Level, actionable_entity: &Entity) {
    let entity = actionable_entity.clone();

    match self.variant {
      LevelEntityVariant::Deletion => {
        let delete_entities = &Box::new(|current_entity: &Entity| {
          !current_entity.is_inside_bounds(entity.x, entity.y, entity.width, entity.height)
        });

        level.background.retain(delete_entities);
        level.indestructible.retain(delete_entities);
        level.destructible.retain(delete_entities);
        level.enemies.retain(delete_entities);
        level.main_character.retain(delete_entities);
        level.effects.retain(delete_entities);
        level.foreground.retain(delete_entities);
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

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
    pub fn new() -> Self {
        Self {
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
    pub fn create_entity(
        &mut self,
        level: &mut Level,
        (x, y, width, height): (f32, f32, u16, u16),
    ) {
        let entity = Entity::new(x, y, width, height).variant(self.entity_variant.clone());

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

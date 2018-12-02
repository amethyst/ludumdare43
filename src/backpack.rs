use amethyst::renderer::SpriteSheetHandle;

pub struct Backpack {
    pub tower_sheet: Option<SpriteSheetHandle>,
    pub bullet_sheet: Option<SpriteSheetHandle>,
}
impl Backpack {
    pub fn new(sheet: SpriteSheetHandle,bullet_sheet: SpriteSheetHandle) -> Self {
        Backpack {
            tower_sheet: Some(sheet),
            bullet_sheet: Some(bullet_sheet),
        }
    }
}
impl Default for Backpack {
    fn default() -> Self {
        Backpack {
            tower_sheet: None,
            bullet_sheet: None,
        }
    }
}
use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,ReadStorage,Entities,Read,ReadExpect};
use amethyst::core::transform::Transform;
use amethyst::input::InputHandler;
use amethyst::renderer::{SpriteRender,ScreenDimensions,MouseButton};

use grid::{Tile,Selected,Hovered};

#[derive(Default)]
pub struct TileSelectionSystem {
    selected: Option<u32>,
}

impl<'s> System<'s> for TileSelectionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s,Tile>,
        ReadStorage<'s,Transform>,
        WriteStorage<'s,SpriteRender>,
        Read<'s, InputHandler<String, String>>,
        ReadExpect<'s,ScreenDimensions>,
        WriteStorage<'s,Selected>,
        ReadStorage<'s,Hovered>,
    );
    
    fn run(&mut self, (entities,mut tiles,transforms,mut sprites,input,dimensions,mut selected,hovered) : Self::SystemData) {
        if input.mouse_button_is_down(MouseButton::Left) {
            if let Some((x,y)) = input.mouse_position(){
                let mut element = (&*entities,&mut tiles,&transforms).join().find(|(_,_,trans)| 
                    x as f32 >= trans.translation.x  && x as f32 <= trans.translation.x + Tile::TILE_SIZE as f32 && 
                    dimensions.height() - y as f32 >= trans.translation.y && dimensions.height() - y as f32 <= trans.translation.y + Tile::TILE_SIZE as f32  );
                // Means mouse is colliding with an element.
                if let Some((e,_,_)) = element {
                    if let Some(id) = self.selected {
                        if e.id() != id {
                            self.selected = None;
                            let ent = entities.entity(id);
                            let sprite = sprites.get_mut(ent).unwrap();
                            selected.remove(ent);
                            sprite.sprite_number = 0;
                        } else {
                            return;
                        }
                    }
                    let sprite = sprites.get_mut(e).unwrap();
                    let _ = selected.insert(e,Selected);
                    sprite.sprite_number = 2;
                    self.selected = Some(e.id());   
                    
                }
            }
        }
    } 
}

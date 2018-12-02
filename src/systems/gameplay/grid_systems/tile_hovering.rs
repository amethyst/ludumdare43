use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,ReadStorage,Entities,Read,ReadExpect};
use amethyst::core::transform::Transform;
use amethyst::input::InputHandler;
use amethyst::renderer::{SpriteRender,ScreenDimensions};
use amethyst::core::cgmath::Vector2;

use grid::{Tile,TileState};

pub struct TileHoverSystem;

impl<'s> System<'s> for TileHoverSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s,Tile>,
        ReadStorage<'s,Transform>,
        WriteStorage<'s,SpriteRender>,
        Read<'s, InputHandler<String, String>>,
        ReadExpect<'s,ScreenDimensions>,
    );
    
    fn run(&mut self, (entities,mut tiles,transforms,mut sprites,input,dimensions) : Self::SystemData) {
        if let Some((x,y)) = input.mouse_position(){
            //println!("{},{}",x,y);
            let mut element = (&mut tiles,&transforms,&mut sprites).join().find(|(_,trans,_)| 
                x as f32 >= trans.translation.x  && x as f32 <= trans.translation.x + Tile::TILE_SIZE as f32 && 
                dimensions.height() - y as f32 >= trans.translation.y && dimensions.height() - y as f32 <= trans.translation.y + Tile::TILE_SIZE as f32 
            );
            if let Some((tile,_,sprite)) = element {
                tile.state = TileState::Hovered;
                sprite.sprite_number = 1;
            } else {
                //(&mut tiles).join().filter(|t| t.state == TileState::Hovered);
            }
        }
    }
}

use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,ReadStorage,Entities,WriteStorage,ReadExpect};
use amethyst::core::transform::{Transform};
use amethyst::renderer::{Hidden,Camera,ScreenDimensions};

use enemy::Enemy;
use grid::Tile;

pub struct HideTileSystem;

impl<'s> System<'s> for HideTileSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s,Tile>,
        WriteStorage<'s,Hidden>,
        ReadStorage<'s,Transform>,
        ReadExpect<'s,ScreenDimensions>,
        ReadStorage<'s,Camera>
    );
    
    fn run(&mut self, (entities,mut tiles,mut hiddens, transforms,dims,camera) : Self::SystemData) {
        let camera_position = {
            if let Some((_,t)) = (&camera, &transforms).join().next() {
                *t.translation().clone()
            } else {
                return;
            }
        };
        let (width,height) = (dims.width(), dims.height());

        for (e,_,t) in (&*entities,&tiles,&transforms).join() {
            let pos = *t.translation();
            let offset = Tile::TILE_SIZE as f32 + 10.0;
            if camera_position.x - offset <= pos.x && camera_position.x + width + offset >= pos.x &&
               camera_position.y - offset <= pos.y && camera_position.y + height + offset >= pos.y {
               if let Some(_) = hiddens.get(e) {
                   hiddens.remove(e);
               }
            } else {
                hiddens.insert(e, Hidden);
            }
        }
    }
}

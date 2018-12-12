use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,Entities,ReadExpect};
use amethyst::renderer::{SpriteRender,Transparent};
use amethyst::core::Transform;
use amethyst::core::transform::components::Parent;

use grid::{Tile,TileType};
use towers::Tower;
use rand::{Rng};
use backpack::Backpack;

pub struct TowerSimpleSystem {
    count: usize,
}
impl Default for TowerSimpleSystem {
    fn default() -> Self {
        Self {
            count: 0,
        }
    }
}

impl<'s> System<'s> for TowerSimpleSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s,Tower>,
        WriteStorage<'s,Tile>,
        WriteStorage<'s,SpriteRender>,
        WriteStorage<'s,Transform>,
        WriteStorage<'s,Parent>,
        WriteStorage<'s,Transparent>,
        ReadExpect<'s,Backpack>,
    );
    
    fn run(&mut self, (entities,mut towers,mut tiles,mut sprites,mut transforms,mut parents,mut transparents,bp) : Self::SystemData) {
        if self.count < 3 {
            // The gun will be a child to the tower base if one
            let mut pos = {
                let mut temp_vec = (&mut tiles,&transforms).join().filter(|(tile,_)| tile.state != TileType::Buildable(true)).collect::<Vec<_>>();
                let idx = rand::thread_rng().gen_range(0,temp_vec.len());
                temp_vec[idx].0.state = TileType::Buildable(true);
                temp_vec[idx].1.translation().clone()
            };

            let sprite = SpriteRender {
                sprite_sheet: bp.tower_sheet.clone().unwrap(),
                sprite_number: 0,
            };
            pos.x += Tile::TILE_SIZE as f32 / 2.0;
            pos.y += Tile::TILE_SIZE as f32 / 2.0;
            pos.z += 0.5;

            entities.build_entity()
                .with(Tower::default(), &mut towers)
                .with(Transform::from(pos), &mut transforms)
                .with(sprite, &mut sprites)
                .with(Transparent::default(), &mut transparents)
                .build();
            
            self.count += 1;
        }
    } 
}

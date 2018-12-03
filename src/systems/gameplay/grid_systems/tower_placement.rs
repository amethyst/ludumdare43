use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,ReadStorage,Entities,Read,ReadExpect,WriteExpect};
use amethyst::core::transform::Transform;
use amethyst::input::InputHandler;
use amethyst::renderer::{SpriteRender,ScreenDimensions,MouseButton};
use amethyst::core::cgmath::Vector2;

use grid::{Tile,Selected};
use tracker::GameTracker;

#[derive(Default)]
pub struct TowerPlacementSystem {
}

impl<'s> System<'s> for TowerPlacementSystem {
    type SystemData = (
        Entities<'s>,
        WriteExpect<'s,GameTracker>,
        WriteStorage<'s,Tile>,
        ReadStorage<'s,Transform>,
        WriteStorage<'s,SpriteRender>,
        Read<'s, InputHandler<String, String>>,
        WriteStorage<'s,Selected>,
    );
    
    fn run(&mut self, (entities,gt,mut tiles,transforms,mut sprites,input,mut selected) : Self::SystemData) {
        
    } 
}

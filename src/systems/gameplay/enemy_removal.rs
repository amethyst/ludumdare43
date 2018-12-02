use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,ReadStorage,Entities};

use enemy::Enemy;

pub struct EnemyRemovalSystem;

impl<'s> System<'s> for EnemyRemovalSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s,Enemy>,
    );
    
    fn run(&mut self, (entities,enemies) : Self::SystemData) {
        for (entity, enemy) in (&*entities,&enemies).join() {
            if enemy.health <= 0.0 {
                let _ = entities.delete(entity);    
            }
        }
    }
}

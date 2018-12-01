use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,ReadStorage,Entities};

use bullet::Bullet;

pub struct BulletRemovalSystem;

impl<'s> System<'s> for BulletRemovalSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s,Bullet>,
    );
    
    fn run(&mut self, (entities,bullets) : Self::SystemData) {
        // Add a collision flag which determines when it should be removed.
        for (entity, _) in (&*entities,&bullets).join() {
            let _ = entities.delete(entity);
        }
    }
}

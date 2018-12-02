use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,ReadStorage,Entities};

use bullet::Bullet;
use collider::CircleCollider;


pub struct BulletRemovalSystem;

impl<'s> System<'s> for BulletRemovalSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s,Bullet>,
        ReadStorage<'s,CircleCollider>,

    );
    
    fn run(&mut self, (entities,bullets,colliders) : Self::SystemData) {
        // Add a collision flag which determines when it should be removed.
        for (entity, _, collider) in (&*entities,&bullets,&colliders).join() {
            if collider.0 {
                let _ = entities.delete(entity);    
            }
        }
    }
}

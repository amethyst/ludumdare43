use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,Read,ReadStorage};
use amethyst::core::transform::Transform;
use amethyst::core::timing::Time;

use towers::Element;
use bullet::Bullet;

pub struct BulletMovementSystem;

impl<'s> System<'s> for BulletMovementSystem {
    type SystemData = (
        Read<'s,Time>,
        ReadStorage<'s,Bullet>,
        WriteStorage<'s,Transform>,
    );
    
    fn run(&mut self, (time,bullets,mut transforms) : Self::SystemData) {
        for (bullet, transform) in (&bullets,&mut transforms).join() {
            transform.translation.x += time.delta_seconds() * Element::DEFAULT_BULLET_SPEED * bullet.0.x;
            transform.translation.y += time.delta_seconds() * Element::DEFAULT_BULLET_SPEED * bullet.0.y;
        }
    }
}

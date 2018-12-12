use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,ReadStorage};
use amethyst::core::transform::Transform;

use bullet::Bullet;
use enemy::Enemy;
use collider::CircleCollider;
use super::targeting::acquire_distance;

pub struct CollisionCheckingSystem;

impl<'s> System<'s> for CollisionCheckingSystem {
    type SystemData = (
        ReadStorage<'s,Bullet>,
        ReadStorage<'s,Transform>,
        WriteStorage<'s,Enemy>,
        WriteStorage<'s,CircleCollider>,
    );
    
    fn run(&mut self, (bullets,transforms,mut enemies,mut colliders) : Self::SystemData) {
        let mut enemy_collisions = {
            (&mut enemies,&transforms,&colliders).join().map(|(enemy,transform,collider)| (enemy,transform.translation(),collider.clone()) ).collect::<Vec<_>>()
        };

        for (bullet,transform,collider) in (&bullets,&transforms,&mut colliders).join() {
            let range = collider.1;
            for (enemy,_,_) in enemy_collisions.iter_mut().filter(|(_,pos,coll)| acquire_distance(*transform.translation(),**pos) as f32 <= range + coll.1) {
                enemy.health -= bullet.damage;
                collider.0 = true;
            }
            
        }
        
    }
}

use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,ReadStorage,Entities};
use amethyst::core::transform::Transform;
use amethyst::core::nalgebra::Vector3;

use towers::Tower;
use enemy::Enemy;

pub struct TargetingSystem;

impl<'s> System<'s> for TargetingSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s,Enemy>,
        WriteStorage<'s,Tower>,
        WriteStorage<'s,Transform>,
    );
    
    fn run(&mut self, (entities,enemies,mut towers,mut transforms) : Self::SystemData) {
        let enemies_vector = {
            (&enemies,&transforms,&*entities).join()
                .map(|(_,t,entitiy)| (entitiy.id(), t.clone()) )
                .collect::<Vec<_>>()
        };

        for (tower,transform) in (&mut towers,&mut transforms).join() {
            //if let None = tower.target {
            //    continue;
            //}
            let range = tower.range;
            
            // Getting the distance between the enemy and the tower
            // and filtering enemies in range.
            let mut temp_vector = enemies_vector
                .iter()
                .map(|(id,t)| (*id,*t.translation(),acquire_distance(*transform.translation(), *t.translation())) )
                .filter(|(_,_,distance)| enemy_is_in_range(*distance as f32,range))
                .collect::<Vec<_>>();

            // Sorting the vector based on distance.
            temp_vector.sort_by_key(|(_,_,dist)| *dist );

            // Getting the closest enemy and setting its 
            // id as the target id.
            if let Some((idx,position)) = temp_vector
                .iter()
                .map(|(id,pos,_)| (*id, *pos))
                .last()
                {
                    transform.face_towards(position,Vector3::new(0.0,0.0,1.0));
                    tower.target = Some(idx);
                } else {
                    tower.target = None;
                }
        }
    }
}

fn enemy_is_in_range(distance: f32, range: f32) -> bool {
    if distance <= range {
        return true;
    }
    false 
}

pub fn acquire_distance(v1: Vector3<f32>, v2: Vector3<f32>) -> u32 {
    ( (v1.x - v2.x).powf(2.0) + (v1.y - v2.y).powf(2.0) ).sqrt() as u32
}
use amethyst::ecs::prelude::{VecStorage,Component};

#[derive(Copy,Clone)]
pub struct CircleCollider(pub bool,pub f32);

impl CircleCollider {
    pub fn new(radius: f32) -> Self {
        CircleCollider(false,radius)
    }
}

impl Default for CircleCollider {
    fn default() -> Self {
        CircleCollider(false,0.0)
    }
}

impl Component for CircleCollider {
    type Storage = VecStorage<Self>;
}
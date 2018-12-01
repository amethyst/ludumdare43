use amethyst::ecs::prelude::{DenseVecStorage,Component};
use amethyst::core::cgmath::Vector2;

#[derive(Clone,Debug)]
pub struct Bullet(pub Vector2<f32>);

impl Default for Bullet{
    fn default() -> Self {
        Bullet(Vector2::new(1.0,1.0))
    }
}

impl Component for Bullet {
    type Storage = DenseVecStorage<Self>;
}


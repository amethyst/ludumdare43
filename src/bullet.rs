use amethyst::ecs::prelude::{DenseVecStorage,Component};
use amethyst::core::nalgebra::Vector2;

use towers::Element;


#[derive(Clone,Debug)]
pub struct Bullet {
    pub factor: Vector2<f32>,
    pub damage: f32,
}
impl Bullet {
    pub fn new(factor: Vector2<f32>,damage: f32) -> Self {
        Self {
            factor,
            damage,
        }
    }
}

impl Default for Bullet{
    fn default() -> Self {
        Self {
            factor: Vector2::new(1.0,1.0),
            damage: Element::DEFAULT_BULLET_DAMAGE,
        }
    }
}

impl Component for Bullet {
    type Storage = DenseVecStorage<Self>;
}


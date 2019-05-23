use amethyst::ecs::prelude::{Component, DenseVecStorage};


pub struct Enemy {
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub fire_speed: f32,
    pub health: i32,
    pub hitbox_width: f32,
    pub hitbox_height: f32,
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}
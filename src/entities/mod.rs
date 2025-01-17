use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Builder, Component, Entities, LazyUpdate, ReadExpect, World},
    renderer::{SpriteRender, Transparent, SpriteSheet},
    assets::{Handle},
};

pub mod blast;
pub mod spaceship;
pub mod enemy;
pub mod enemy_spawner;
pub mod explosion;
pub mod items;
pub mod side_panel;
pub mod background;
pub mod consumable;
pub mod defense;
pub mod status_bar;
pub mod status_unit;
pub mod gamemaster;
pub mod store;
pub mod planet;

pub use self::{
    blast::{fire_blast, fire_double_blast},
    spaceship::initialise_spaceship,
    enemy::{spawn_enemy},
    enemy_spawner::{initialise_enemy_spawner },
    explosion::{spawn_explosion},
    items::spawn_item,
    side_panel::initialise_side_panels,
    background::initialise_background,
    consumable::spawn_consumable,
    defense::initialise_defense,
    status_bar::initialise_status_bars,
    status_unit::spawn_status_unit,
    gamemaster::initialise_gamemaster,
    store::{initialise_store},
    planet::{initialise_planet},
};

fn spawn_sprite_entity<T: Component + Send + Sync>(
    entities: &Entities,
    sprite_render: SpriteRender,
    item_comp: T,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);
    lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(item_comp)
        .with(local_transform)
        .with(Transparent)
        .build();
}

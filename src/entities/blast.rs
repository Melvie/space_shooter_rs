use amethyst::{
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
    core::{
        transform::Transform,
        math::Vector3,
    },
};
use crate::{
    components::Blast,
    resources::SpriteResource,
};
use rand::{thread_rng, Rng};

const HITBOX_RADIUS: f32 = 2.0;
const VELOCITY_FACTOR: f32 = 0.5; //amount that the spaceship/enemy's velocity affects the blast's velocity
const CRIT_SPRITE_NUM: usize = 2;
const POISON_SPRITE_NUM: usize = 3;

pub fn fire_blast(entities: &Entities, blast_resource: &ReadExpect<SpriteResource>, sprite_number: usize, fire_position: Vector3<f32>, mut damage: f32, x_velocity: f32, y_velocity: f32, speed: f32, allied: bool, crit_chance: f32, poison_chance: f32, lazy_update: &ReadExpect<LazyUpdate>) {
    let blast_entity: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_translation(fire_position);

    let mut sprite_render = SpriteRender {
        sprite_sheet: blast_resource.blasts_sprite_sheet.clone(),
        sprite_number: sprite_number,
    };

    //roll for crit
    let mut poison_damage = 0.0;
    let crit_roll = thread_rng().gen::<f32>();
    let poison_roll = thread_rng().gen::<f32>();
    if crit_roll < crit_chance {
        sprite_render.sprite_number = CRIT_SPRITE_NUM;
        damage *= 2.0;
    }else if poison_roll < poison_chance {
        sprite_render.sprite_number = POISON_SPRITE_NUM;
        poison_damage = damage/100.0;
    }

    lazy_update.insert(blast_entity, sprite_render);
    lazy_update.insert(blast_entity, Blast {speed: speed, hitbox_radius: HITBOX_RADIUS, damage: damage, poison_damage: poison_damage, x_velocity: x_velocity, y_velocity: y_velocity, velocity_factor: VELOCITY_FACTOR, allied: allied});
    lazy_update.insert(blast_entity, local_transform);
    lazy_update.insert(blast_entity, Transparent);

}

pub fn fire_double_blast(entities: &Entities, blast_resource: &ReadExpect<SpriteResource>, sprite_number: usize, fire_position: Vector3<f32>, mut damage: f32, x_velocity: f32, y_velocity: f32, speed: f32, allied: bool, crit_chance: f32, poison_chance: f32, lazy_update: &ReadExpect<LazyUpdate>) {
    let blast_entity_1: Entity = entities.create();
    let blast_entity_2: Entity = entities.create();

    let mut local_transform_1 = Transform::default();
    local_transform_1.set_translation(Vector3::new(fire_position[0] + 5.0, fire_position[1], fire_position[2]));

    let mut local_transform_2 = Transform::default();
    local_transform_2.set_translation(Vector3::new(fire_position[0] - 5.0, fire_position[1], fire_position[2]));

    let mut sprite_render = SpriteRender {
        sprite_sheet: blast_resource.blasts_sprite_sheet.clone(),
        sprite_number: sprite_number,
    };

    //roll for crit
    let mut poison_damage = 0.0;
    let crit_roll = thread_rng().gen::<f32>();
    let poison_roll = thread_rng().gen::<f32>();
    if crit_roll < crit_chance {
        sprite_render.sprite_number = CRIT_SPRITE_NUM;
        damage *= 2.0;
    }else if poison_roll < poison_chance {
        sprite_render.sprite_number = POISON_SPRITE_NUM;
        poison_damage = damage/100.0;
    }

    lazy_update.insert(blast_entity_1, sprite_render.clone());
    lazy_update.insert(blast_entity_1, Blast {speed: speed, hitbox_radius: HITBOX_RADIUS, damage: damage, poison_damage: poison_damage, x_velocity: x_velocity, y_velocity: y_velocity, velocity_factor: VELOCITY_FACTOR, allied: allied});
    lazy_update.insert(blast_entity_1, local_transform_1);
    lazy_update.insert(blast_entity_1, Transparent);

    lazy_update.insert(blast_entity_2, sprite_render.clone());
    lazy_update.insert(blast_entity_2, Blast {speed: speed, hitbox_radius: HITBOX_RADIUS, damage: damage, poison_damage: poison_damage, x_velocity: x_velocity, y_velocity: y_velocity, velocity_factor: VELOCITY_FACTOR, allied: allied});
    lazy_update.insert(blast_entity_2, local_transform_2);
    lazy_update.insert(blast_entity_2, Transparent);

}

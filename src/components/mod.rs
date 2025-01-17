use amethyst::{
    core::{
        transform::Transform,
        math::Vector3,
    },
};

mod blast;
mod spaceship;
mod enemy;
mod explosion;
mod item;
mod consumable;
mod defense;
mod status_bar;
mod spawner;
mod gamemaster;
mod store;
mod planet;

pub use self::{
    blast::Blast,
    spaceship::{Spaceship},
    enemy::{Enemy, EnemyType, EnemySpawnerTag},
    explosion::Explosion,
    item::{Item},
    consumable::{Consumable},
    defense::Defense,
    status_bar::{StatusBar, StatusType},
    spawner::{Spawner, SpawnProbabilities, choose_random_name},
    gamemaster::{GameMaster, Phase, PhaseType},
    store::Store,
    planet::Planet,
};

const BLAST_Z: f32 = 0.1;

pub trait Rigidbody {
    fn current_velocity_x(&self) -> f32;
    fn current_velocity_y(&self) -> f32;
    fn acceleration_x(&self) -> f32;
    fn acceleration_y(&self) -> f32;
    fn deceleration_x(&self) -> f32;
    fn deceleration_y(&self) -> f32;
    fn max_speed(&self) -> f32;
    fn knockback_max_speed(&self) -> f32;
    fn set_current_velocity_y(&mut self, value: f32);
    fn set_current_velocity_x(&mut self, value: f32);

    fn update_position(&self, transform: &mut Transform, dt: f32) {
        transform.set_translation_x(transform.translation().x + self.current_velocity_x()*dt);
        transform.set_translation_y(transform.translation().y + self.current_velocity_y()*dt);
    }

    fn accelerate_x(&mut self, direction: f32) {
        self.set_current_velocity_x(self.current_velocity_x() + (direction * self.acceleration_x()));
    }

    fn accelerate_y(&mut self, direction: f32) {
        self.set_current_velocity_y(self.current_velocity_y() + (direction * self.acceleration_y()));
    }

    fn decelerate_x(&mut self, direction: f32) {
        self.set_current_velocity_x(self.current_velocity_x() + (direction *  self.deceleration_x()));
    }

    fn decelerate_y(&mut self, direction: f32) {
        self.set_current_velocity_y(self.current_velocity_y() + (direction *  self.deceleration_y()));
    }

    fn accelerate(&mut self, direction_x: f32, direction_y: f32) {
        if (direction_x > 0.0 && self.current_velocity_x() < self.max_speed()) || (direction_x < 0.0 && self.current_velocity_x() > (-1.0 * self.max_speed())) {
            self.accelerate_x(direction_x);
        } else if direction_x == 0.0 && self.current_velocity_x() > 0.0 {
            self.decelerate_x(-1.0);
        } else if direction_x == 0.0 && self.current_velocity_x() < 0.0 {
            self.decelerate_x(1.0);
        }

        if (direction_y > 0.0 && self.current_velocity_y() < self.max_speed()) || (direction_y < 0.0 && self.current_velocity_y() > (-1.0 * self.max_speed())) {
            self.accelerate_y(direction_y);
        } else if direction_y == 0.0 && self.current_velocity_y() > 0.0 {
            self.decelerate_y(-1.0);
        } else if direction_y == 0.0 && self.current_velocity_y() < 0.0 {
            self.decelerate_y(1.0);
        }
    }

    fn limit_knockback(&mut self) {
        if self.current_velocity_x() > self.knockback_max_speed() {
            self.set_current_velocity_x(self.knockback_max_speed());
        }
        if self.current_velocity_x() < -1.0 * self.knockback_max_speed() {
            self.set_current_velocity_x(-1.0 * self.knockback_max_speed());
        }
        if self.current_velocity_y() > self.knockback_max_speed() {
            self.set_current_velocity_y(self.knockback_max_speed());
        }
        if self.current_velocity_y() < -1.0 * self.knockback_max_speed() {
            self.set_current_velocity_y(-1.0 * self.knockback_max_speed());
        }
    }

    fn limit_speed(&mut self) {
        if self.current_velocity_x() > self.max_speed() {
            self.decelerate_x(-1.0);
        }
        if self.current_velocity_x() < -1.0 * self.max_speed() {
            self.decelerate_x(1.0);
        }
        if self.current_velocity_y() > self.max_speed() {
            self.decelerate_y(-1.0);
        }
        if self.current_velocity_y() < -1.0 * self.max_speed() {
            self.decelerate_y(1.0);
        }
    }
}

pub trait Fires {
    fn fire_reset_timer(&self) -> f32;
    fn fire_speed(&self) -> f32;
    fn set_fire_reset_timer(&mut self, value: f32);

    fn fire_cooldown(&mut self, transform: &mut Transform, offset: f32, fire: bool, dt: f32) -> Option<Vector3<f32>> {
        if self.fire_reset_timer() > 0.0 {
            self.set_fire_reset_timer(self.fire_reset_timer() - dt);
            return None;
        }else if fire {
            self.set_fire_reset_timer(self.fire_speed());
            let fire_position = Vector3::new(
                transform.translation()[0], transform.translation()[1] + offset, BLAST_Z,
            );
            return Some(fire_position);
        }else {
            return None
        }
    }
}

pub trait Living {
    fn health(&self) -> f32;
    fn max_health(&self) -> f32;
    fn set_health(&mut self, value: f32);
    fn set_max_health(&mut self, value: f32);

    fn constrain_health(&mut self) {
        if self.health() < 0.0 {
            self.set_health(0.0);
        }else if self.health() > self.max_health() {
            self.set_health(self.max_health());
        }
    }
}
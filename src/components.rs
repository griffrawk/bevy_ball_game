use bevy::prelude::*;
use rand::prelude::*;

use crate::constants::*;

#[derive(Component)]
pub struct Player {
    pub size: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self { size: PLAYER_SIZE }
    }
}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
    pub size: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            direction: Vec2::new(
                rand::thread_rng().gen_range(-1.0..1.0),
                rand::thread_rng().gen_range(-1.0..1.0),
            )
            .normalize(),
            size: ENEMY_SIZE,
        }
    }
}

#[derive(Component)]
pub struct Star {}

#[derive(Component)]
pub struct LockBlock {}

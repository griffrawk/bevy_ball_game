use bevy::prelude::*;

use crate::game::constants::*;

#[derive(Component)]
pub struct Player {
    pub size: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self { size: PLAYER_SIZE }
    }
}

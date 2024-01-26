pub mod components;
mod resources;
mod systems;

use bevy::prelude::*;

use resources::EnemySpawnTimer;
use systems::*;

pub struct EnemyPlugin {}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(Startup, spawn_enemies)
            .add_systems(Update, enemy_movement)
            .add_systems(Update, confine_enemy_movement)
            .add_systems(Update, update_enemy_direction)
            .add_systems(Update, enemy_hit_player)
            .add_systems(Update, spawn_enemies_over_time);
    }
}

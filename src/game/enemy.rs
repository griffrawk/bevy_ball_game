pub mod components;
mod resources;
mod systems;

use bevy::prelude::*;

use crate::AppState;
use resources::EnemySpawnTimer;
use systems::*;

use super::SimulationState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    confine_enemy_movement,
                    update_enemy_direction,
                    spawn_enemies_over_time,
                )
                    .chain()
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(Update, enemy_hit_player)
            // exit state systems
            .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}

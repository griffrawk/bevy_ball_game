pub mod components;
mod resources;
mod systems;

use bevy::prelude::*;

use crate::AppState;
use resources::EnemySpawnTimer;
use resources::EnemyAnimationTimer;
use systems::*;

use super::SimulationState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .init_resource::<EnemyAnimationTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    animate_enemy_sprite,
                    spawn_enemies_over_time,
                    enemy_hit_player,
                )
                    .chain()
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // exit state systems
            .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}

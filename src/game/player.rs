pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;

use resources::AnimationTimer;

use super::SimulationState;
use crate::AppState;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<AnimationTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    animate_player_sprite,
                    confine_player_movement,
                    player_catch_star,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
                    .chain(),
            )
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}

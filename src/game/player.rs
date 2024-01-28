pub mod components;
mod systems;

use bevy::prelude::*;

use super::SimulationState;
use crate::AppState;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(
                Update,
                (player_movement, confine_player_movement, player_catch_star)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
                    .chain(),
            )
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}

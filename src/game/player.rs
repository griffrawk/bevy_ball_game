pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;

use super::SimulationState;
use crate::AppState;
use resources::AnimationTimer;
use systems::*;

#[derive(States, Debug, Clone, Copy, Eq, Hash, PartialEq, Default)]
pub enum PlayerState {
    #[default]
    Paused,
    Walking,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<AnimationTimer>()
            .add_state::<PlayerState>()
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    confine_player_movement,
                    animate_player_sprite.run_if(in_state(PlayerState::Walking)),
                    player_catch_star,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
                    .chain()
            )
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}


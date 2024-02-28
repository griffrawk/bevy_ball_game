mod constants;
pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
mod systems;

use bevy::prelude::*;

use crate::{events::GameOver, AppState};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

#[derive(States, Clone, Copy, Eq, Hash, PartialEq, Default, Debug)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_plugins((EnemyPlugin, ScorePlugin, StarPlugin))
            .add_plugins(PlayerPlugin)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}


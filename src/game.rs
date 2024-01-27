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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Clone, Copy, Eq, Hash, PartialEq, Default, Debug)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}

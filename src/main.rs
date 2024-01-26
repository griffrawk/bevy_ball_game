mod constants;
mod enemy;
mod events;
mod player;
mod score;
mod star;
mod systems;

use bevy::prelude::*;

use enemy::EnemyPlugin;
use events::GameOver;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}

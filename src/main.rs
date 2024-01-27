mod constants;
mod events;
mod game;
mod main_menu;
mod systems;

use bevy::prelude::*;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // My plugins
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        // Starup
        .add_systems(Startup, spawn_camera)
        // Systems
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, Hash, PartialEq, Default)]
pub enum AppState{
    #[default]
    MainMenu,
    Game,
    GameOver,
}

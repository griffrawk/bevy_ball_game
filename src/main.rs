mod events;
mod game;
mod main_menu;
mod systems;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

use game::player::components::PlayerAssets;
use game::enemy::components::EnemyAssets;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

#[derive(States, Debug, Clone, Copy, Eq, Hash, PartialEq, Default)]
pub enum AppState {
    #[default]
    AssetLoading,
    MainMenu,
    Game,
    GameOver,
}

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .init_state::<AppState>()
        // Load asset texture atlas
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading)
                .continue_to_state(AppState::MainMenu)
                .load_collection::<PlayerAssets>()
                .load_collection::<EnemyAssets>()
        )
        // My plugins
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        // Startup
        .add_systems(Startup, spawn_camera)
        // Systems
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_menu_state)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}


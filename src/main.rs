mod events;
mod game;
mod main_menu;
mod systems;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_state::<AppState>()
        // Load asset texture atlas
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading)
            .continue_to_state(AppState::MainMenu)
            .load_collection::<MyAssets>(),
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

#[derive(States, Debug, Clone, Copy, Eq, Hash, PartialEq, Default)]
pub enum AppState {
    #[default]
    AssetLoading,
    MainMenu,
    Game,
    GameOver,
}


// todo move this somewhere more player oriented... game/player/resources.rs maybe
#[derive(AssetCollection, Resource)]
struct MyAssets {
    // if the sheet would have padding, you could set that with `padding_x` and `padding_y`.
    // if there would be space between the top left corner of the sheet and the first sprite, you could configure that with `offset_x` and `offset_y`
    #[asset(texture_atlas(tile_size_x = 96., tile_size_y = 99., columns = 8, rows = 1))]
    // you can configure the sampler for the sprite sheet image
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/female_adventurer_sheet.png")]
    female_adventurer: Handle<TextureAtlas>,
}
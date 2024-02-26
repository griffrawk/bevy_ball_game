use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::game::constants::*;

#[derive(Component)]
pub struct Player {
    pub size: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self { size: PLAYER_SIZE }
    }
}

// bevy_asset_loader 
#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(texture_atlas_layout(tile_size_x = 96.0, tile_size_y = 99.0, columns = 8, rows = 1))]
    pub female_adventurer_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/female_adventurer_sheet.png")]
    pub female_adventurer: Handle<Image>,
}

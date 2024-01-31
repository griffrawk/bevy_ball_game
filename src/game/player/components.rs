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

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(texture_atlas(tile_size_x = 96., tile_size_y = 99., columns = 8, rows = 1))]
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/female_adventurer_sheet.png")]
    pub female_adventurer: Handle<TextureAtlas>,
}

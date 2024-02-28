use bevy::prelude::*;
use rand::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::game::constants::*;

#[derive(Component, Debug)]
pub struct Enemy {
    pub direction: Vec2,
    pub speed: f32,
    pub size: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            direction: Vec2::new(
                rand::thread_rng().gen_range(-1.0..=1.0),
                rand::thread_rng().gen_range(-1.0..=1.0),
            )
            .normalize(),
            speed: rand::thread_rng().gen_range(ENEMY_LOWER_SPEED..=ENEMY_UPPER_SPEED),
            size: ENEMY_SIZE,
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct EnemyAssets {
    #[asset(texture_atlas_layout(tile_size_x = 64.0, tile_size_y = 64.0, columns = 8, rows = 3))]
    pub enemy_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "sprites/slime_jiggle.png")]
    pub enemy: Handle<Image>,
}

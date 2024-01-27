pub mod resources;
mod systems;

use bevy::prelude::*;

use resources::Score;
use systems::update_score;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Update, update_score);
    }
}

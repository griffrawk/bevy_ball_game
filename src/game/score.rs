pub mod resources;
mod systems;

use bevy::prelude::*;

use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            // todo some more runif and score malarkey EP9 @ 32:00
            .add_systems(
                Update,
                (update_score, update_high_score, high_scores_updated),
            );
    }
}

use bevy::prelude::*;

use crate::score::resources::*;

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}
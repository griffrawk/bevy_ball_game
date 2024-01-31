use bevy::prelude::*;

#[derive(Resource)]
pub struct AnimationTimer {
    pub timer: Timer,
}

impl Default for AnimationTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

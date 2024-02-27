use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerAnimationTimer {
    pub timer: Timer,
}

impl Default for PlayerAnimationTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

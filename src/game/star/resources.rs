use bevy::prelude::*;

pub const STAR_SPAWN_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct StarSpwanTimer {
    pub timer: Timer,
}

impl Default for StarSpwanTimer {
    fn default() -> StarSpwanTimer {
        StarSpwanTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

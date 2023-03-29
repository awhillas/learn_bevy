use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub const STARS_COUNT: usize = 10;
pub const STAR_SIZE: f32 = 30.0;

use super::AppState;
use super::SimulationState;
use resources::StarSpwanTimer;
use systems::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpwanTimer>()
            // On Enter State
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (tick_star_spawn_timer, star_spwaner)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // On Exit State
            .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)));
    }
}

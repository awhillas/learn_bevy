pub mod components;
mod systems;

use super::AppState;
use super::SimulationState;
use bevy::prelude::*;
use systems::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinmentSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(MovementSystemSet.before(ConfinmentSystemSet))
            // .add_startup_system(spawn_player)
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            // .add_systems((player_movement, confine_movement).chain())
            .add_system(
                player_movement
                    .in_set(MovementSystemSet)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_system(
                confine_movement
                    .in_set(ConfinmentSystemSet)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // .add_system(enemy_collision_detection)
            // .add_system(star_collision_detection)
            .add_systems(
                (enemy_collision_detection, star_collision_detection)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}

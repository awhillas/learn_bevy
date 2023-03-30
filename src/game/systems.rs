use bevy::prelude::*;

use super::SimulationState;

pub fn pause_simulation(mut next_sim_state: ResMut<NextState<SimulationState>>) {
    next_sim_state.set(SimulationState::Paused)
}

pub fn resume_simulation(mut next_sim_state: ResMut<NextState<SimulationState>>) {
    next_sim_state.set(SimulationState::Running)
}

pub fn toggle_simulation(
    mut next_sim_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<Input<KeyCode>>,
    sim_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if sim_state.0 == SimulationState::Running {
            next_sim_state.set(SimulationState::Paused);
            println!("Game Paused...")
        }
        if sim_state.0 == SimulationState::Paused {
            next_sim_state.set(SimulationState::Running);
            println!("... Game resumed!")
        }
    }
}

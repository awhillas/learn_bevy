use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::events::*;
use crate::game::SimulationState;
use crate::AppState;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn transition_to_game_state(
    mut next_app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.0 != AppState::Game {
            next_app_state.set(AppState::Game);
            println!("Entering Game state")
        }
    }
}

pub fn transition_to_main_menu_state(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.0 != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            next_sim_state.set(SimulationState::Paused);
            println!("Entering MainMenu state")
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit)
    }
}

pub fn handle_game_over(
    mut app_state: ResMut<NextState<AppState>>,
    mut game_over_event_reader: EventReader<GameOver>,
) {
    for event in game_over_event_reader.iter() {
        println!("Your final Score was {}", event.score.to_string());
        app_state.set(AppState::Game);
    }
}

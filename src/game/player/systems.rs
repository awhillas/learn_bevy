use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Player;
use crate::events::GameOver;
use crate::game::enemy::components::Enemy;
use crate::game::enemy::ENEMY_SIZE;
use crate::game::score::resources::*;
use crate::game::star::components::Star;
use crate::game::star::STAR_SIZE;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/PNG/Default/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize()
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let min_x = 0.0 + half_player_size;
        let max_x = window.width() - half_player_size;
        let min_y = 0.0 + half_player_size;
        let max_y = window.height() - half_player_size;

        let mut translation = player_transform.translation;
        if translation.x < min_x {
            translation.x = min_x;
        } else if translation.x > max_x {
            translation.x = max_x;
        }

        if translation.y < min_y {
            translation.y = min_y;
        } else if translation.y > max_y {
            translation.y = max_y;
        }

        player_transform.translation = translation;
    }
}

pub fn star_collision_detection(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    audio: Res<Audio>,
    assets: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(&player_transform) = player_query.get_single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            if distance < PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0 {
                println!("Player hit a star");
                audio.play(assets.load("audio/interface/pluck_001.ogg"));
                commands.entity(star_entity).despawn();
                score.value += 100;
            }
        }
    }
}

pub fn enemy_collision_detection(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    audio: Res<Audio>,
    assets: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, &player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game over :(");
                audio.play(assets.load("audio/sci-fi/explosionCrunch_000.ogg"));
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value })
            }
        }
    }
}
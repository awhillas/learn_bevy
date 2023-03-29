use rand::prelude::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::enemy::components::*; // or
use crate::game::enemy::resources::*;
use crate::game::enemy::{ENEMY_COUNT, ENEMY_SIZE, ENEMY_SPEED};

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // let half_size = ENEMY_SIZE / 2.0;

    for _ in 0..ENEMY_COUNT {
        let x = (random::<f32>() * 0.8 + 0.1) * window.width();
        let y = (random::<f32>() * 0.8 + 0.1) * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/PNG/Default/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn despwan_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    assets: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let min_x = 0.0 + half_enemy_size;
    let max_x = window.width() - half_enemy_size;
    let min_y = 0.0 + half_enemy_size;
    let max_y = window.height() - half_enemy_size;

    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;
        let mut translation = transform.translation;

        // Change direction

        if translation.x < min_x || translation.x > max_x {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < min_y || translation.y > max_y {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        // Move to within bounds if outside of it

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

        transform.translation = translation;

        // Play sound efxs

        if direction_changed {
            let sound_1 = assets.load("audio/sci-fi/laserLarge_000.ogg");
            let sound_2 = assets.load("audio/sci-fi/laserLarge_001.ogg");

            let sound_effect = if random::<f32>() > 0.5 {
                sound_1
            } else {
                sound_2
            };
            audio.play(sound_effect);
        }
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spwaner: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spwaner.timer.tick(time.delta());
}

pub fn enemy_spawner(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawner: Res<EnemySpawnTimer>,
) {
    if enemy_spawner.timer.finished() {
        let window = window_query.get_single().unwrap();

        // let half_size = ENEMY_SIZE / 2.0;

        let x = (random::<f32>() * 0.8 + 0.1) * window.width();
        let y = (random::<f32>() * 0.8 + 0.1) * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/PNG/Default/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

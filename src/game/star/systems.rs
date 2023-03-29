use rand::prelude::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Star;
use super::resources::*;
use super::STARS_COUNT;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..STARS_COUNT {
        let x = (random::<f32>() * 0.8 + 0.1) * window.width();
        let y = (random::<f32>() * 0.8 + 0.1) * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/PNG/Default/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn despawn_stars(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    for entity in star_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn tick_star_spawn_timer(mut star_spawner: ResMut<StarSpwanTimer>, time: Res<Time>) {
    star_spawner.timer.tick(time.delta());
}

pub fn star_spwaner(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
    star_spwaner: Res<StarSpwanTimer>,
) {
    if star_spwaner.timer.finished() {
        // timer has hit zero
        let window = window_query.get_single().unwrap();

        let x = (random::<f32>() * 0.8 + 0.1) * window.width();
        let y = (random::<f32>() * 0.8 + 0.1) * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: assets.load("sprites/PNG/Default/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

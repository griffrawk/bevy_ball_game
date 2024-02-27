use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use super::components::Enemy;
use super::resources::EnemySpawnTimer;
use super::resources::EnemyAnimationTimer;
use crate::game::constants::*;
use crate::events::GameOver;
use crate::game::enemy::components::EnemyAssets;

// For collision detection
use crate::game::player::components::Player;

use crate::game::score::resources::Score;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // asset_server: Res<AssetServer>,
    enemy_assets: Res<EnemyAssets>,
) {
    let window = window_query.get_single().unwrap();
    // randomly spread around
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        // commands.spawn((
        //     SpriteBundle {
        //         // randomly in window
        //         transform: Transform::from_xyz(random_x, random_y, 0.0),
        //         texture: asset_server.load("sprites/ball_red_large.png"),
        //         ..default()
        //     },
        //     Enemy::default(),
        // ));

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: enemy_assets.enemy.clone(),
                ..default()
            },
            TextureAtlas::from(enemy_assets.enemy_layout.clone()),
            Enemy::default(),
        ));
    
    }
}

pub fn despawn_enemies(
    mut commands: Commands, 
    enemy_query: Query<Entity, With<Enemy>>
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>, 
    time: Res<Time>
) {
    
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * enemy.speed * time.delta_seconds();
    }
}

#[allow(unused)]
pub fn update_enemy_direction(
    // mut commands: Commands, // needed for the sfx since 0.1?
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // asset_server: Res<AssetServer>,  // for sfx
) {
    let window = window_query.get_single().unwrap();

    for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
        let half_enemy_size = enemy.size / 2.0;
        let x_min = 0.0 + half_enemy_size;
        let x_max = window.width() - half_enemy_size;
        let y_min = 0.0 + half_enemy_size;
        let y_max = window.height() - half_enemy_size;

        let mut direction_changed = false;
        let translation = enemy_transform.translation;
        // If out of bounds in any dimension, flip the direction in that dimension
        // le ge suggested in video comments otherwise they get stuck
        if translation.x <= x_min || translation.x >= x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y <= y_min || translation.y >= y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        // play sfx
        // if direction_changed {
        //     // pick a random pluck
        //     let sound_effect = if random::<f32>() > 0.5 {
        //         asset_server.load("audio/pluck_001.ogg")
        //     } else {
        //         asset_server.load("audio/pluck_002.ogg")
        //     };
        //     Commented out. Getting very old, very quick...
        //     commands.spawn(AudioBundle {
        //         source: sound_effect,
        //         settings: PlaybackSettings::DESPAWN,
        //         ..default()
        //     });
        // }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>, // filtered. Transform for an Enemy
    window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    // todo If the screen dimensions change then the bounds don't behave correctly for
    //  some reason.
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_HITBOX / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut enemy_transform in enemy_query.iter_mut() {
        let mut translation = enemy_transform.translation;
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }
        enemy_transform.translation = translation;
    }
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // asset_server: Res<AssetServer>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    enemy_assets: Res<EnemyAssets>,

) {
    if enemy_spawn_timer.timer.tick(time.delta()).finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        // commands.spawn((
        //     SpriteBundle {
        //         transform: Transform::from_xyz(random_x, random_y, 0.0),
        //         texture: asset_server.load("sprites/ball_red_large.png"),
        //         ..default()
        //     },
        //     Enemy::default(),
        // ));

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: enemy_assets.enemy.clone(),
                ..default()
            },
            TextureAtlas::from(enemy_assets.enemy_layout.clone()),
            Enemy::default(),
        ));

    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    // Entity is just a u32, so it can be copied, not borrowed.
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    // for sfx
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        // check each enemy to see if in collision with player
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            // Hopefully these are inlined by the compiler...
            // but might change to get the sizes from the components
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_HITBOX / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game Over!");
                // let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/explosionCrunch_000.ogg"),
                    // source: sound_effect,
                    settings: PlaybackSettings::DESPAWN,
                });
                // Player 1 Go Boom...
                commands.entity(player_entity).despawn();
                // send final score to whatever system is listening for GameOver
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn animate_enemy_sprite(
    time: Res<Time>,
    mut enemy_animation_timer: ResMut<EnemyAnimationTimer>,
    mut sprites_to_animate: Query<&mut TextureAtlas, With<Enemy>>,
) {
    // sprites_to_animate is empty, query not working
    if enemy_animation_timer.timer.tick(time.delta()).finished() {
        for mut sprite in &mut sprites_to_animate {
            sprite.index = (sprite.index + 1) % 8;
        }
    }
}

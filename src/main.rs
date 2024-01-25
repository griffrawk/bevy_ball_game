// bevy_ball_game - Jacques

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_SIZE: f32 = 64.0;
const NUMBER_OF_ENEMIES: usize = 4;
const ENEMY_SPEED: f32 = 200.0;
const ENEMY_SIZE: f32 = 64.0;
const NUMBER_OF_STARS: usize = 10;
const STAR_SIZE: f32 = 30.0;
const STAR_SPAWN_TIME: f32 = 1.0;
const ENEMY_SPAWN_TIME: f32 = 5.0;

fn main() {
    // All the systems are added very verbosely, but they can be grouped, and can be conditional as well.
    // It's happened like this because of the evolvment of the videos they are taken from.
    // Coming soon (ep8), the big refactor.
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_stars)
        .add_systems(Update, player_movement)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, confine_sprite_movement)
        // .add_systems(Update, confine_player_movement)
        // .add_systems(Update, confine_enemy_movement)
        .add_systems(Update, update_enemy_direction)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, player_catch_star)
        .add_systems(Update, update_score)
        // .add_systems(Update, tick_star_spawn_timer)
        .add_systems(Update, spawn_enemies_over_time)
        .add_systems(Update, spawn_stars_over_time)
        .add_systems(Update, bevy::window::close_on_esc)
        // or...
        // .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}

#[derive(Component)]
struct Player {
    size: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self { size: PLAYER_SIZE }
    }
}

#[derive(Component)]
struct Enemy {
    direction: Vec2,
    size: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            direction: Vec2::new(
                rand::thread_rng().gen_range(-1.0..1.0),
                rand::thread_rng().gen_range(-1.0..1.0),
            )
            .normalize(),
            size: ENEMY_SIZE,
        }
    }
}

#[derive(Component)]
struct Star {}

#[derive(Component)]
struct LockBlock {}

#[derive(Resource)]
struct Score {
    value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[derive(Resource)]
struct StarSpawnTimer {
    timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
struct EnemySpawnTimer {
    timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Event)]
struct GameOver {
    score: u32,
}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            // middle of window
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player::default(),
    ));
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        // middle of window
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

#[allow(unused)]
fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    // randomly spread around
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                // randomly in window
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy::default(),
        ));
    }
}

#[allow(unused)]
fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    // randomly spread around
    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                // randomly in window
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

#[allow(unused)]
fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    // todo If the screen dimensions change then the bounds don't behave correctly for
    //  some reason.
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;
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
        player_transform.translation = translation;
    }
}

fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

fn update_enemy_direction(
    mut commands: Commands, // needed for the sfx since 0.1?
    mut enemy_query: Query<(&Transform, &mut Enemy)>, // tuple
    window_query: Query<&Window, With<PrimaryWindow>>, // filtered
    // audio: Res<Audio>,         // old doesn't work
    asset_server: Res<AssetServer>,
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
        if direction_changed {
            // pick a random pluck
            let sound_effect = if random::<f32>() > 0.5 {
                asset_server.load("audio/pluck_001.ogg")
            } else {
                asset_server.load("audio/pluck_002.ogg")
            };
            // audio.play(sound_effect);        // old doesn't work
            commands.spawn(AudioBundle {
                source: sound_effect,
                settings: PlaybackSettings::DESPAWN,
                ..default()
            });
        }
    }
}

#[allow(unused)]
fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>, // filtered. Transform for an Enemy
    window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    // todo If the screen dimensions change then the bounds don't behave correctly for
    //  some reason.
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
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

fn confine_sprite_movement(
    mut sprite_query: Query<(&mut Transform, AnyOf<(&Enemy, &Player)>)>,
    window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    // Slightly more generic confine, in that it works for both Enemy and Player, using the sizes
    // stored in the component. That makes it a bit more flexible if a third component comes along with
    // a different size.
    let window = window_query.get_single().unwrap();

    for (mut sprite_transform, (enemy, player)) in sprite_query.iter_mut() {
        let mut half_sprite_size = 0.0;
        // It'll be one or the other...
        if let Some(sprite) = enemy {
            half_sprite_size = sprite.size / 2.0;
        }
        if let Some(sprite) = player {
            half_sprite_size = sprite.size / 2.0;
        }

        let x_min = 0.0 + half_sprite_size;
        let x_max = window.width() - half_sprite_size;
        let y_min = 0.0 + half_sprite_size;
        let y_max = window.height() - half_sprite_size;

        let mut translation = sprite_transform.translation;
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
        sprite_transform.translation = translation;
    }
}

fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    // Entity is just a u32 so it can be copied, not borrowed.
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
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
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game Over!");
                // let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/explosionCrunch_000.ogg"),
                    // source: sound_effect,
                    settings: PlaybackSettings::DESPAWN,
                    ..default()
                });
                // Player 1 Go Boom...
                commands.entity(player_entity).despawn();
                // send final score to whatever system is listening for GameOver
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

fn player_catch_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    // Entity is just a u32 so it can be copied...!
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        // check each star to see if in collision with player
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            // Hopefully these are inlined by the compiler...
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;
            if distance < player_radius + star_radius {
                println!("Player hit star!");
                score.value += 1;
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/laserLarge_000.ogg"),
                    settings: PlaybackSettings::DESPAWN,
                    ..default()
                });
                // Catch a falling star
                commands.entity(star_entity).despawn();
            }
        }
    }
}

fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    time: Res<Time>,
) {
    if star_spawn_timer.timer.tick(time.delta()).finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));

        // can_i_do_this(commands, asset_server); // yes I can
    }
}

fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    if enemy_spawn_timer.timer.tick(time.delta()).finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy::default(),
        ));
    }
}

#[allow(unused)]
fn can_i_do_this(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(100.0, 100.0, 0.0),
            texture: asset_server.load("sprites/block_locked_square.png"),
            ..default()
        },
        LockBlock {},
    ));
}

#[allow(unused)]
fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut app_exit_event_writer: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.read() {
        println!("Final Score: {}", event.score);
    }
}

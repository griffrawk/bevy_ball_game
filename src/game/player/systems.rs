use bevy::prelude::*;

use bevy::input::gamepad::GamepadButton;
use bevy::window::PrimaryWindow;

use super::components::Player;
use super::resources::PlayerAnimationTimer;
use super::PlayerState;
use crate::game::constants::*;
use crate::game::player::components::PlayerAssets;
use crate::game::score::resources::Score;
use crate::game::star::components::Star;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // asset_server: Res<AssetServer>,
    player_assets: Res<PlayerAssets>,
    // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let window = window_query.get_single().unwrap();
    // The bevy standard, static sprite
    // commands.spawn((
    //     SpriteBundle {
    //         // middle of window
    //         transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    //         texture: asset_server.load("sprites/ball_blue_large.png"),
    //         ..default()
    //     },
    //     Player::default(),
    // ));

    // Using bevy_asset_loader
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: player_assets.female_adventurer.clone(),
            ..default()
        },
        TextureAtlas::from(player_assets.female_adventurer_layout.clone()),
        Player::default(),
    ));

    // The bevy native way
    // handle to the image sheet
    // let texture: Handle<Image> = asset_server.load("sprites/female_adventurer_sheet.png");
    // Describe the spritesheet
    // let layout = TextureAtlasLayout::from_grid(Vec2::new(96.0, 96.0), 8, 1, None, None);
    // add the layout to the resources
    // let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // commands.spawn((
    //     SpriteSheetBundle {
    //         texture,
    //         atlas: TextureAtlas {
    //             layout: texture_atlas_layout,
    //             index: 0,
    //         },
    //         transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    //         ..default()
    //     },
    //     Player::default(),
    // ));
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    player_state: Res<State<PlayerState>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    time: Res<Time>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }

        // It works!
        for gamepad in gamepads.iter() {
            let left_stick_x = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
                .unwrap();
            let left_stick_y = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
                .unwrap();
            if left_stick_x < -0.1 {
                // info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
                direction += Vec3::new(-1.0, 0.0, 0.0)
            }
            if left_stick_x > 0.1 {
                // info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
                direction += Vec3::new(1.0, 0.0, 0.0)
            }
            if left_stick_y < -0.1 {
                // info!("{:?} LeftStickY value is {}", gamepad, left_stick_y);
                direction += Vec3::new(0.0, -1.0, 0.0)
            }
            if left_stick_y > 0.1 {
                // info!("{:?} LeftStickY value is {}", gamepad, left_stick_y);
                direction += Vec3::new(0.0, 1.0, 0.0)
            }
        }

        // Are we moving?
        if direction.length() > 0.0 {
            direction = direction.normalize();
            if *player_state.get() == PlayerState::Paused {
                next_player_state.set(PlayerState::Walking);
            }
        } else if *player_state.get() == PlayerState::Walking {
            next_player_state.set(PlayerState::Paused);
        }

        player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
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

pub fn player_catch_star(
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
                score.value += 1;
                // Pow!
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/laserLarge_000.ogg"),
                    settings: PlaybackSettings::DESPAWN,
                });
                // Catch a falling star
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn animate_player_sprite(
    time: Res<Time>,
    mut player_animation_timer: ResMut<PlayerAnimationTimer>,
    mut sprites_to_animate: Query<&mut TextureAtlas, With<Player>>,
) {
    if player_animation_timer.timer.tick(time.delta()).finished() {
        for mut sprite in &mut sprites_to_animate {
            sprite.index = (sprite.index + 1) % 8;
        }
    }
}

#[allow(dead_code)]
pub fn gamepad_system(
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
) {
    for gamepad in gamepads.iter() {
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            info!("{:?} just pressed South", gamepad);
        } else if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::South))
        {
            info!("{:?} just released South", gamepad);
        }

        let right_trigger = button_axes
            .get(GamepadButton::new(
                gamepad,
                GamepadButtonType::RightTrigger2,
            ))
            .unwrap();
        if right_trigger.abs() > 0.1 {
            info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
        }

        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() > 0.1 {
            info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
        }
        let left_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
        if left_stick_y.abs() > 0.1 {
            info!("{:?} LeftStickY value is {}", gamepad, left_stick_y);
        }
    }
}

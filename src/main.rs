// bevy_ball_game - Jacques
pub mod systems;
pub mod components;
pub mod events;
pub mod resources;
pub mod constants;

use bevy::prelude::*;

use crate::systems::*;
use crate::events::*;
use crate::resources::*;

fn main() {
    // All the systems are added very verbosely, but they can be grouped, and can be conditional as well.
    // It's happened like this because of the evolvment of the videos they are taken from.
    // Coming soon (ep7), the big refactor.
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

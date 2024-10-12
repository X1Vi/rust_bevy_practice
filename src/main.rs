use std::thread::spawn;

use bevy::ecs::bundle;
use bevy::render::camera::ScalingMode;
use bevy::{
    prelude::*,
};
use bevy_rapier2d::prelude::*;

mod scripts {
    pub mod camera;
    pub mod player;
    pub mod obstacle;
}



// camera scripts
use scripts::camera::camera_follow_system;
use scripts::camera::zoom_in_orthographic_projection;

// player scripts
use scripts::player::player_movement;
use scripts::player::Player;
use scripts::player::lock_player_rotation;

// obstacles 

use scripts::obstacle::spawn_obstacle;
use scripts::obstacle::setup_obstacle_spawn;
use scripts::obstacle::despawn_obstacles;
fn main() {
    App::new()
        // Add default Bevy plugins
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest(), 
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup, setup_obstacle_spawn))
        .add_systems(Update, (zoom_in_orthographic_projection, player_movement, despawn_obstacles, camera_follow_system))
        .add_systems(Update, spawn_obstacle)
        .add_systems(Update, spawn_obstacle)
        .add_systems(Update, spawn_obstacle)
        .add_systems(Update, lock_player_rotation)
        .run();
}


/* Set the active collision types inside of a system. */
fn modify_collider_active_collision_types(mut active_types: Query<&mut ActiveCollisionTypes>) {
    for mut active_types in active_types.iter_mut() {
        *active_types = (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC);
    }
}

// This system runs once at the start and sets up a camera
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    commands.spawn(Camera2dBundle {
        ..Default::default()

    });
    commands
    .spawn(RigidBody::Dynamic) // Correctly chain spawn method
    .insert(SpriteBundle {
        texture: asset_server.load("Kenny/Tiles/tile_0120.png"),
        transform: Transform::from_xyz(100.0, 100.0, 0.0),
        ..Default::default()
    })
    .insert(Player {
        direction: Vec3::ZERO,
        speed: 80.0,
    })
    .insert(Collider::capsule(
        Vec2::new(0.0, 0.0), // Center position of the capsule
        Vec2::new(0.0, 4.0), // This should be adjusted to 0.0 on the x-axis, and height of 16.0 on the y-axis for a vertical capsule
        8.0, // Radius of the capsule, half of 16.0
    ))
    .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC)
    .insert(GravityScale(0.0));
}


// This system prints a message every frame
fn print_message() {
    println!("Hello, Bevy!");
}


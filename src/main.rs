use std::thread::spawn;

use bevy::ecs::bundle;
use bevy::render::camera::ScalingMode;
use bevy::{
    prelude::*,
};
use bevy_rapier2d::prelude::{Collider, RigidBody};

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

// obstacles 

use scripts::obstacle::spawn_obstacle;
use scripts::obstacle::setup_obstacle_spawn;
use scripts::obstacle::despawn_obstacles;
use scripts::obstacle::insert_WindowSize_resource;
fn main() {
    App::new()
        // Add default Bevy plugins
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest(), 
        ))
        .add_systems(Startup, setup_obstacle_spawn)
        .add_systems(Startup, print_message)
        .add_systems(Startup, setup)
        .add_systems(Update, zoom_in_orthographic_projection)
        .add_systems(Update, player_movement)
        .add_systems(Update, spawn_obstacle)
        .add_systems(Update, despawn_obstacles)
        .add_systems(Update, camera_follow_system)
        .run();
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
    .spawn(RigidBody::KinematicVelocityBased) // Correctly chain spawn method
    .insert(SpriteBundle {
        texture: asset_server.load("Kenny/Tiles/tile_0120.png"),
        transform: Transform::from_xyz(100.0, 100.0, 0.0),
        ..Default::default()
    })
    .insert(Player {
        direction: Vec3::ZERO,
        speed: 80.0,
    })
    .insert(Collider::capsule(Vec2::new(0.0, 0.0), Vec2::new(16.0, 16.0), 6.0)); // Replace comma with a period

    
}


// This system prints a message every frame
fn print_message() {
    println!("Hello, Bevy!");
}


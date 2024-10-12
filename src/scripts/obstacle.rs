use bevy::{ecs::query, log::tracing_subscriber::fmt::time, prelude::*, render::view::window, transform::commands, window::{PrimaryWindow, WindowResized}};
use bevy_rapier2d::prelude::*;
use std::time::Duration;
use rand::Rng;
use super::player::Player;




const SPAWN_TIME : u64 = 1;
const DE_SPAWN_TIME : u64 = 4;
const SPAWN_DISTANCE_X_RIGHT: f32 = 150.0;



#[derive(Resource)]
pub struct WindowSize {
    width : f32,
    height: f32
}

#[derive(Component)]
pub struct Obstacle {
    timer: Timer,
    is_timer_started: bool
}

#[derive(Resource)]
pub struct ObstacleSpawnTimer(Timer);

pub fn insert_WindowSize_resource(
    windows_size : Res<WindowSize>,
    mut commands: Commands
){
    commands.insert_resource(
        WindowSize {width: windows_size.width, height: windows_size.height}
    );
}

pub fn spawn_obstacle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawn_timer: ResMut<ObstacleSpawnTimer>,
    player_transform_query: Query<&Transform, With<Player>>, 
    windows_query : Query<&Window, With<PrimaryWindow>>

) {
    spawn_timer.0.tick(time.delta());

    if spawn_timer.0.finished() {
        for player_position in player_transform_query.iter() {
            let mut rng = rand::thread_rng();

            for window in windows_query.iter() {

                commands.spawn(SpriteBundle {
                    texture: asset_server.load("Kenny/Tiles/tile_0040.png"),
                    transform: Transform::from_xyz(player_position.translation.x + SPAWN_DISTANCE_X_RIGHT, rng.gen_range(100.0..(window.height() - 100.0)), 0.0),
                    ..Default::default()
                })
                .insert((RigidBody::Fixed, GlobalTransform::default()))
                .insert(Obstacle {timer : Timer::new(Duration::from_secs(DE_SPAWN_TIME), TimerMode::Once), is_timer_started:false})
                .insert(Collider::capsule(Vec2::new(0.0, 0.0), Vec2::new(0.0, 4.0), 8.0))
                .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC);
                }
            }

        spawn_timer.0.reset();
    }
}

// Add this to your app setup
pub fn setup_obstacle_spawn(mut commands: Commands) {
    commands.insert_resource(ObstacleSpawnTimer(Timer::new(Duration::from_secs(SPAWN_TIME), TimerMode::Repeating)));
}

pub fn despawn_obstacles(
    mut commands: Commands,                             // Commands to despawn entities
    mut obstacles_query: Query<(Entity, &mut Obstacle)>, // Mutable query to access and modify `Obstacle`
    time: Res<Time>,                                    // Time resource to track time progression
) {
    for (entity, mut obstacle) in obstacles_query.iter_mut() {
        // Start the timer if not already started
        if !obstacle.is_timer_started {
            obstacle.is_timer_started = true;
            obstacle.timer.reset();
        }

        // Update the timer each frame
        obstacle.timer.tick(time.delta());

        // Despawn the obstacle entity once the timer finishes
        if obstacle.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
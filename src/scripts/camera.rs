use bevy::{
    prelude::*, render::camera::ScalingMode
};

use super::player::Player;


pub fn camera_follow_system(
    player_query: Query<&Transform, With<Player>>, // Query for the player's transform
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>, // Query for the camera's transform
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut camera_transform in camera_query.iter_mut() {
            // Set the camera's position to the player's position
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}

pub fn zoom_in(mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.z -= 100.0;

        warn!("{}", transform.translation.z);
    }
}
pub fn zoom_in_orthographic_projection(
    mut query: Query<&mut OrthographicProjection, With<Camera>>, 
    time: Res<Time>
) {
    for mut orthographic_projection in query.iter_mut() {
        // Adjust the scaling mode with a zoom-in effect based on the delta time
        // let zoom_factor: f32 = 1.0 + time.delta_seconds();
        orthographic_projection.scale = 0.5;
        // println!("zoom factor -> {}", zoom_factor);
    }
}
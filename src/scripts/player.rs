use bevy::{
    prelude::*
};


#[derive(Component)]
pub struct  Player {
    pub direction : Vec3,
    pub speed : f32
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform), With<Player>>, // Mutable access to Player
    time: Res<Time>,
) {
    for (mut player, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO; // Reset direction for each frame

        // Check keyboard input for movement
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0; // Move up
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0; // Move down
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0; // Move left
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0; // Move right
        }

        // Normalize direction and apply speed only if there's input
        if direction.length_squared() > 0.0 {
            direction = direction.normalize(); // Normalize direction vector
            // Update the player's position based on direction and speed
            transform.translation += direction * player.speed * time.delta_seconds(); // Accumulate movement
        }
        
        // Set player direction to zero if no keys are pressed
        player.direction = direction; // Store the current direction (will be Vec3::ZERO if no keys pressed)
    
    }
}
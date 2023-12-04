use bevy::prelude::*;

use crate::states::GameState;

// for z-ordering
const CAMERA_LEVEL: f32 = 20.0;

pub struct Camera2dPlugin;

impl Plugin for Camera2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, zoom_camera.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, CAMERA_LEVEL),
            scale: Vec3::ONE,
            ..default()
        },
        ..default()
    });
}

fn zoom_camera(mut camera_query: Query<&mut Transform, With<Camera2d>>, keys: Res<Input<KeyCode>>) {
    let mut camera_transform = camera_query
        .get_single_mut()
        .expect("More than one Camera2d!");

    if keys.just_pressed(KeyCode::PageUp) {
        camera_transform.scale += Vec3::new(0.2, 0.2, 0.0);
    } else if keys.just_pressed(KeyCode::PageDown) {
        camera_transform.scale -= Vec3::new(0.2, 0.2, 0.0);
    }
}

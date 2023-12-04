use bevy::prelude::*;

// for z-ordering
const CAMERA_LEVEL: f32 = 20.0;

pub struct Camera2dPlugin;

impl Plugin for Camera2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
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

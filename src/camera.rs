use bevy::{prelude::*, window::PrimaryWindow};

use crate::states::GameState;

// for z-ordering
const CAMERA_LEVEL: f32 = 20.0;

/// The position of the cursor, in world coordinates.
#[derive(Resource, Debug, Default)]
pub struct CursorPosition(pub Vec2);

pub struct Camera2dPlugin;

impl Plugin for Camera2dPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, get_cursor_pos)
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

fn get_cursor_pos(
    mut pos: ResMut<CursorPosition>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let (camera, camera_transform) = camera_query.single();

    let window = window_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        pos.0 = world_position;
    }
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

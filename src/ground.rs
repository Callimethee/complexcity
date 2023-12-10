use bevy::prelude::*;
use std::ops::Range;

use crate::asset_loader::AssetHandles;

const TILE_SCALE: Vec3 = Vec3::new(1.0, 1.0, 0.0);
const TILE_SIZE: Vec3 = Vec3::new(16.0, 16.0, 0.0);
const TILES_RANGE_X: Range<i32> = -120..120;
const TILES_RANGE_Y: Range<i32> = -100..100;

#[derive(Component, Debug)]
struct GroundTile;

#[derive(Bundle)]
struct TileBundle {
    tile: GroundTile,
    sprite: SpriteBundle,
}

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_floor);
    }
}

/// Spawns all floor tiles in a grid
fn spawn_floor(mut commands: Commands, asset_handles: Res<AssetHandles>) {
    for x_pos in TILES_RANGE_X {
        for y_pos in TILES_RANGE_Y {
            spawn_tile(x_pos, y_pos, &mut commands, &asset_handles);
        }
    }
}

fn spawn_tile(x_pos: i32, y_pos: i32, commands: &mut Commands, asset_handles: &Res<AssetHandles>) {
    commands.spawn(TileBundle {
        tile: GroundTile,
        sprite: SpriteBundle {
            texture: asset_handles.street.clone(),
            transform: Transform {
                translation: Vec3::new(
                    x_pos as f32 * TILE_SIZE.x * TILE_SCALE.x,
                    y_pos as f32 * TILE_SIZE.y * TILE_SCALE.y,
                    0.0,
                ),
                scale: TILE_SCALE,
                ..default()
            },
            ..default()
        },
    });
}

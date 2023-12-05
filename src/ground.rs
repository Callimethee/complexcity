use bevy::prelude::*;
use std::ops::Range;

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

fn spawn_floor(mut commands: Commands, asset_server: Res<AssetServer>) {
    for x_pos in TILES_RANGE_X {
        for y_pos in TILES_RANGE_Y {
            spawn_tile(x_pos, y_pos, &mut commands, &asset_server);
        }
    }
}

fn spawn_tile(x_pos: i32, y_pos: i32, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(TileBundle {
        tile: GroundTile,
        sprite: SpriteBundle {
            texture: asset_server.load("street.png"),
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

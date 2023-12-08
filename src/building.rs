use bevy::prelude::*;

use crate::{asset_loader::AssetHandles, drag::Draggable, states::GameState};

const MIN_DISTANCE: f32 = 30.0;

const BUILDING_LEVEL: f32 = 1.0;
// Buildings sprite dimensions
const HOUSE_SIZE: Vec2 = Vec2::new(48.0, 48.0);
const FORUM_SIZE: Vec2 = Vec2::new(80.0, 96.0);
const CINEMA_SIZE: Vec2 = Vec2::new(48.0, 42.0);
const HOSPITAL_SIZE: Vec2 = Vec2::new(48.0, 96.0);
const POOL_SIZE: Vec2 = Vec2::new(64.0, 54.0);
const RESTAURANT_SIZE: Vec2 = Vec2::new(64.0, 48.0);
const CREATIVE_SIZE: Vec2 = Vec2::new(68.0, 42.0);
const UNDERGROUND_SIZE: Vec2 = Vec2::new(32.0, 48.0);

#[derive(Debug, Component, PartialEq)]
enum BuildingType {
    House,
    Forum,
    Cinema,
    Hospital,
    Pool,
    Restaurant,
    Creative,
    Underground,
}

#[derive(Bundle)]
struct BuildingBundle {
    b_type: BuildingType,
    sprite: SpriteBundle,
    draggable: Draggable,
}

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (trigger_spawn, hitbox_follow, destack_buildings).run_if(in_state(GameState::Playing)),
        );
    }
}

fn trigger_spawn(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::H) {
        spawn_building(BuildingType::House, &mut commands, &asset_handles)
    }
    if keys.just_pressed(KeyCode::F) {
        spawn_building(BuildingType::Forum, &mut commands, &asset_handles)
    }
}

fn spawn_building(
    b_type: BuildingType,
    commands: &mut Commands,
    asset_handles: &Res<AssetHandles>,
) {
    let handle = match b_type {
        BuildingType::House => asset_handles.house.clone(),
        BuildingType::Forum => asset_handles.forum.clone(),
        BuildingType::Cinema => asset_handles.cinema.clone(),
        BuildingType::Hospital => asset_handles.hospital.clone(),
        BuildingType::Pool => asset_handles.pool.clone(),
        BuildingType::Restaurant => asset_handles.restaurant.clone(),
        BuildingType::Creative => asset_handles.creative.clone(),
        BuildingType::Underground => asset_handles.underground.clone(),
    };
    let top_right = get_size_from_type(&b_type);

    commands.spawn(BuildingBundle {
        b_type,
        sprite: SpriteBundle {
            texture: handle,
            transform: Transform {
                translation: Vec3::new(top_right.x / 2.0, top_right.y / 2.0, BUILDING_LEVEL),
                ..default()
            },
            ..default()
        },
        draggable: Draggable {
            bottom_left: Vec2::ZERO,
            top_right,
            being_dragged: false,
        },
    });
}

fn hitbox_follow(mut draggables_query: Query<(&mut Draggable, &BuildingType, &Transform)>) {
    for (mut draggable, b_type, transform) in &mut draggables_query {
        let building_size = get_size_from_type(b_type);
        draggable.bottom_left.x = transform.translation.x - building_size.x / 2.0;
        draggable.bottom_left.y = transform.translation.y - building_size.y / 2.0;
        draggable.top_right.x = transform.translation.x + building_size.x / 2.0;
        draggable.top_right.y = transform.translation.y + building_size.y / 2.0;
    }
}

fn get_size_from_type(b_type: &BuildingType) -> Vec2 {
    match b_type {
        BuildingType::House => HOUSE_SIZE,
        BuildingType::Forum => FORUM_SIZE,
        BuildingType::Cinema => CINEMA_SIZE,
        BuildingType::Hospital => HOSPITAL_SIZE,
        BuildingType::Pool => POOL_SIZE,
        BuildingType::Restaurant => RESTAURANT_SIZE,
        BuildingType::Creative => CREATIVE_SIZE,
        BuildingType::Underground => UNDERGROUND_SIZE,
    }
}

fn destack_buildings(
    mut buildings_query: Query<(&BuildingType, &mut Transform)>,
    buttons: Res<Input<MouseButton>>,
) {
    if !buttons.pressed(MouseButton::Left) {
        let mut combinations = buildings_query.iter_combinations_mut();
        while let Some([(b_type_1, mut transform_1), (_b_type_2, mut transform_2)]) =
            combinations.fetch_next()
        {
            let b_size = get_size_from_type(b_type_1);
            if transform_1.translation.distance(transform_2.translation) < MIN_DISTANCE {
                transform_1.translation.x -= b_size.x / 2.0;
                transform_2.translation.x += b_size.x / 2.0;
            }
        }
    }
}

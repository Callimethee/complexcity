use bevy::prelude::*;

use crate::{
    asset_loader::AssetHandles,
    debug::TEXT_SIZE,
    drag::{Draggable, Interactable},
    states::GameState,
};

/// The minimum distance between two buildings.
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
const TREE_SIZE: Vec2 = Vec2::new(32.0, 48.0);
const LAMP_SIZE: Vec2 = Vec2::new(10.0, 46.0);

#[derive(Resource, Default, Debug)]
struct BuildingAvailable {
    house: bool,
    forum: bool,
    cinema: bool,
    hospital: bool,
    pool: bool,
    restaurant: bool,
    creative: bool,
}

#[derive(Resource, Debug)]
struct HouseTimer(Timer);

#[derive(Resource, Debug)]
struct ForumTimer(Timer);

#[derive(Resource, Debug)]
struct CinemaTimer(Timer);

#[derive(Resource, Debug)]
struct HospitalTimer(Timer);

#[derive(Resource, Debug)]
struct PoolTimer(Timer);

#[derive(Resource, Debug)]
struct RestaurantTimer(Timer);

#[derive(Resource, Debug)]
struct CreativeTimer(Timer);

#[derive(Component, Debug)]
struct BuildingInfoText;

#[derive(Debug, Component, PartialEq)]
pub enum BuildingType {
    House,
    Forum,
    Cinema,
    Hospital,
    Pool,
    Restaurant,
    Creative,
    Underground,
    Tree,
    Lamp,
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
        app.insert_resource(BuildingAvailable { ..default() })
            .insert_resource(HouseTimer(Timer::from_seconds(11.0, TimerMode::Repeating)))
            .insert_resource(ForumTimer(Timer::from_seconds(55.0, TimerMode::Repeating)))
            .insert_resource(CinemaTimer(Timer::from_seconds(99.0, TimerMode::Repeating)))
            .insert_resource(HospitalTimer(Timer::from_seconds(
                80.0,
                TimerMode::Repeating,
            )))
            .insert_resource(PoolTimer(Timer::from_seconds(109.0, TimerMode::Repeating)))
            .insert_resource(RestaurantTimer(Timer::from_seconds(
                27.0,
                TimerMode::Repeating,
            )))
            .insert_resource(CreativeTimer(Timer::from_seconds(
                66.0,
                TimerMode::Repeating,
            )))
            .add_systems(OnEnter(GameState::Playing), spawn_info_text)
            .add_systems(
                Update,
                (
                    update_info_text,
                    trigger_spawn,
                    hitbox_follow,
                    destack_buildings,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                OnExit(GameState::Playing),
                (cleanup_buildings, cleanup_text),
            );
    }
}

fn spawn_info_text(mut commands: Commands) {
    let text_style = TextStyle {
        font_size: TEXT_SIZE,
        ..default()
    };

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("H: House", text_style.clone()),
            TextSection::new("/", text_style.clone()),
            TextSection::new("R: Restaurant", text_style.clone()),
            TextSection::new("/", text_style.clone()),
            TextSection::new("F: Forum", text_style.clone()),
            TextSection::new("/", text_style.clone()),
            TextSection::new("C: Cinema", text_style.clone()),
            TextSection::new("/", text_style.clone()),
            TextSection::new("O: Hospital", text_style.clone()),
            TextSection::new("/", text_style.clone()),
            TextSection::new("P: Pool", text_style.clone()),
            TextSection::new("/", text_style.clone()),
            TextSection::new("E: Creative supplies", text_style),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::VMin(1.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }),
        BuildingInfoText,
    ));
}

fn update_info_text(
    mut text_query: Query<&mut Text, With<BuildingInfoText>>,
    available: Res<BuildingAvailable>,
) {
    let mut text = text_query.single_mut();

    let red = TextStyle {
        font_size: TEXT_SIZE,
        color: Color::RED,
        ..default()
    };
    let green = TextStyle {
        font_size: TEXT_SIZE,
        color: Color::GREEN,
        ..default()
    };

    if available.house {
        text.sections[0].style = green.clone();
    } else {
        text.sections[0].style = red.clone();
    }
    if available.restaurant {
        text.sections[2].style = green.clone();
    } else {
        text.sections[2].style = red.clone();
    }
    if available.forum {
        text.sections[4].style = green.clone();
    } else {
        text.sections[4].style = red.clone();
    }
    if available.cinema {
        text.sections[6].style = green.clone();
    } else {
        text.sections[6].style = red.clone();
    }
    if available.hospital {
        text.sections[8].style = green.clone();
    } else {
        text.sections[8].style = red.clone();
    }
    if available.pool {
        text.sections[10].style = green.clone();
    } else {
        text.sections[10].style = red.clone();
    }
    if available.creative {
        text.sections[12].style = green;
    } else {
        text.sections[12].style = red;
    }
}

fn trigger_spawn(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut available: ResMut<BuildingAvailable>,
    mut t_house: ResMut<HouseTimer>,
    mut t_forum: ResMut<ForumTimer>,
    mut t_cinema: ResMut<CinemaTimer>,
    mut t_hospital: ResMut<HospitalTimer>,
    mut t_pool: ResMut<PoolTimer>,
    mut t_restaurant: ResMut<RestaurantTimer>,
    mut t_creative: ResMut<CreativeTimer>,
) {
    if !available.house {
        t_house.0.tick(time.delta());
    }
    if !available.forum {
        t_forum.0.tick(time.delta());
    }
    if !available.cinema {
        t_cinema.0.tick(time.delta());
    }
    if !available.hospital {
        t_hospital.0.tick(time.delta());
    }
    if !available.pool {
        t_pool.0.tick(time.delta());
    }
    if !available.restaurant {
        t_restaurant.0.tick(time.delta());
    }
    if !available.creative {
        t_creative.0.tick(time.delta());
    }

    available.house = t_house.0.just_finished();
    available.forum = t_forum.0.just_finished();
    available.cinema = t_cinema.0.just_finished();
    available.hospital = t_hospital.0.just_finished();
    available.pool = t_pool.0.just_finished();
    available.restaurant = t_restaurant.0.just_finished();
    available.creative = t_creative.0.just_finished();

    if keys.just_pressed(KeyCode::H) && available.house {
        spawn_building(BuildingType::House, &mut commands, &asset_handles);
        available.house = false;
    }
    if keys.just_pressed(KeyCode::F) && available.forum {
        spawn_building(BuildingType::Forum, &mut commands, &asset_handles);
        available.forum = false;
    }
    if keys.just_pressed(KeyCode::C) && available.cinema {
        spawn_building(BuildingType::Cinema, &mut commands, &asset_handles);
        available.cinema = false;
    }
    if keys.just_pressed(KeyCode::O) && available.hospital {
        spawn_building(BuildingType::Hospital, &mut commands, &asset_handles);
        available.hospital = false;
    }
    if keys.just_pressed(KeyCode::P) && available.pool {
        spawn_building(BuildingType::Pool, &mut commands, &asset_handles);
        available.pool = false;
    }
    if keys.just_pressed(KeyCode::R) && available.restaurant {
        spawn_building(BuildingType::Restaurant, &mut commands, &asset_handles);
        available.restaurant = false;
    }
    if keys.just_pressed(KeyCode::E) && available.creative {
        spawn_building(BuildingType::Creative, &mut commands, &asset_handles);
        available.creative = false;
    }
    if keys.just_pressed(KeyCode::T) {
        spawn_building(BuildingType::Tree, &mut commands, &asset_handles);
    }
    if keys.just_pressed(KeyCode::L) {
        spawn_building(BuildingType::Lamp, &mut commands, &asset_handles);
    }
    // if keys.just_pressed(KeyCode::U) {
    //     spawn_building(BuildingType::Underground, &mut commands, &asset_handles)
    // }
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
        BuildingType::Tree => asset_handles.tree.clone(),
        BuildingType::Lamp => asset_handles.lamp.clone(),
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
            interact: Interactable {
                bottom_left: Vec2::ZERO,
                top_right,
            },
            being_dragged: false,
        },
    });
}

fn hitbox_follow(mut draggables_query: Query<(&mut Draggable, &BuildingType, &Transform)>) {
    for (mut draggable, b_type, transform) in &mut draggables_query {
        let building_size = get_size_from_type(b_type);
        draggable.interact.bottom_left.x = transform.translation.x - building_size.x / 2.0;
        draggable.interact.bottom_left.y = transform.translation.y - building_size.y / 2.0;
        draggable.interact.top_right.x = transform.translation.x + building_size.x / 2.0;
        draggable.interact.top_right.y = transform.translation.y + building_size.y / 2.0;
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
        BuildingType::Tree => TREE_SIZE,
        BuildingType::Lamp => LAMP_SIZE,
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

fn cleanup_buildings(mut commands: Commands, buildings_query: Query<Entity, With<BuildingType>>) {
    for entity in &buildings_query {
        commands.entity(entity).despawn_recursive();
    }
}

fn cleanup_text(mut commands: Commands, text_query: Query<Entity, With<BuildingInfoText>>) {
    for entity in &text_query {
        commands.entity(entity).despawn_recursive();
    }
}

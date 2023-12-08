use bevy::prelude::*;

use crate::{asset_loader::AssetHandles, movement::MovementDir, states::GameState};

pub const SPRITE_SCALE: Vec3 = Vec3::new(1.0, 1.0, 0.0);
const PERSON_LEVEL: f32 = 2.0;

#[derive(Resource)]
pub struct UsedPersons {
    pub list: Vec<i32>,
}

#[derive(Resource, Debug)]
struct SpawnTimer(Timer);

#[derive(Debug, Default, Clone, Copy)]
pub enum PersonState {
    #[default]
    Idle,
    LookingForFood,
    LookingForShelter,
}

#[derive(Component, Debug, Default, Clone)]
pub struct Person {
    pub id: i32,
    pub hunger: f32,
    pub satisfaction: f32,
    pub state: PersonState,
    pub movement_direction: MovementDir,
    pub movement_vector: Vec3,
    pub liked: Vec<i32>,
    pub disliked: Vec<i32>,
}

#[derive(Bundle)]
struct PersonBundle {
    person: Person,
    sprite: SpriteBundle,
}

pub struct PersonPlugin;

impl Plugin for PersonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UsedPersons { list: vec![] })
            .insert_resource(SpawnTimer(Timer::from_seconds(7.0, TimerMode::Repeating)))
            .add_systems(OnEnter(GameState::Playing), spawn_first_person)
            .add_systems(
                Update,
                (spawn_person, update_satisfactions).run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                PostUpdate,
                update_liked_disliked.run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_persons);
    }
}

fn spawn_first_person(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    mut used_ids: ResMut<UsedPersons>,
) {
    commands.spawn(PersonBundle {
        person: Person {
            id: 0,
            hunger: 50.0,
            satisfaction: 50.0,
            state: PersonState::Idle,
            movement_direction: MovementDir::PlusBoth,
            movement_vector: Vec3::ZERO,
            ..default()
        },
        sprite: SpriteBundle {
            texture: asset_handles.person.clone(),
            transform: Transform {
                scale: SPRITE_SCALE,
                translation: Vec3::new(0.0, 0.0, PERSON_LEVEL),
                ..default()
            },
            ..default()
        },
    });
    used_ids.list.push(0);
}

fn spawn_person(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    mut used_ids: ResMut<UsedPersons>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    if spawn_timer.0.tick(time.delta()).just_finished() && used_ids.list.len() <= 200 {
        let largest_id = used_ids.list.iter().max();
        let mut available_id = 0;
        if let Some(val) = largest_id {
            available_id = val + 1;
        }
        commands.spawn(PersonBundle {
            person: Person {
                id: available_id,
                hunger: 50.0,
                satisfaction: 50.0,
                state: PersonState::Idle,
                movement_direction: MovementDir::PlusBoth,
                movement_vector: Vec3::ZERO,
                ..default()
            },
            sprite: SpriteBundle {
                texture: asset_handles.person.clone(),
                transform: Transform {
                    scale: SPRITE_SCALE,
                    translation: Vec3::new(0.0, 0.0, PERSON_LEVEL),
                    ..default()
                },
                ..default()
            },
        });
        used_ids.list.push(available_id);
    }
}

fn update_liked_disliked(mut person_query: Query<&mut Person>, used_ids: Res<UsedPersons>) {
    for mut person in &mut person_query {
        for id in &used_ids.list {
            if !person.liked.contains(id) && !person.disliked.contains(id) {
                let likes_this_one: bool = rand::random();
                if likes_this_one {
                    person.liked.push(*id);
                } else {
                    person.disliked.push(*id);
                }
            }
        }
    }
}

fn update_satisfactions(mut person_query: Query<&mut Person>) {
    for mut person in &mut person_query {
        person.satisfaction = person.hunger;
    }
}

fn cleanup_persons(
    mut commands: Commands,
    person_query: Query<Entity, With<Person>>,
    mut used_ids: ResMut<UsedPersons>,
) {
    for entity in &person_query {
        commands.entity(entity).despawn_recursive();
    }
    used_ids.list = vec![];
}

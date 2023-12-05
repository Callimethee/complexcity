use bevy::prelude::*;

use crate::{movement::MovementDir, states::GameState};

const SPRITE_SCALE: Vec3 = Vec3::new(1.0, 1.0, 0.0);

#[derive(Resource)]
struct UsedPersons {
    list: Vec<i32>,
}

#[derive(Resource, Debug)]
struct SpawnTimer(Timer);

#[derive(Debug, Default)]
pub enum PersonState {
    #[default]
    Idle,
    LookingForFood,
}

#[derive(Component, Debug, Default)]
pub struct Person {
    pub id: i32,
    pub hunger: i32,
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
            .add_systems(Update, (spawn_person).run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), cleanup_persons);
    }
}

fn spawn_first_person(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut used_ids: ResMut<UsedPersons>,
) {
    commands.spawn(PersonBundle {
        person: Person {
            id: 0,
            hunger: 0,
            state: PersonState::Idle,
            movement_direction: MovementDir::PlusBoth,
            movement_vector: Vec3::ZERO,
            ..default()
        },
        sprite: SpriteBundle {
            texture: asset_server.load("Adam_idle_front.png"),
            transform: Transform {
                scale: SPRITE_SCALE,
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..default()
            },
            ..default()
        },
    });
    used_ids.list.push(0);
}

fn spawn_person(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
                hunger: 0,
                state: PersonState::Idle,
                movement_direction: MovementDir::PlusBoth,
                movement_vector: Vec3::ZERO,
                ..default()
            },
            sprite: SpriteBundle {
                texture: asset_server.load("Adam_idle_front.png"),
                transform: Transform {
                    scale: SPRITE_SCALE,
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..default()
                },
                ..default()
            },
        });
        used_ids.list.push(available_id);
    }
}

fn cleanup_persons(mut commands: Commands, person_query: Query<Entity, With<Person>>) {
    for entity in &person_query {
        commands.entity(entity).despawn_recursive();
    }
}

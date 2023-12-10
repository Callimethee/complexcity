use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    asset_loader::AssetHandles, building::BuildingType, drag::Interactable, movement::MovementDir,
    states::GameState,
};

pub const SPRITE_SCALE: Vec3 = Vec3::new(1.0, 1.0, 0.0);
const SPRITE_SIZE: Vec3 = Vec3::new(16.0, 23.0, 0.0);

// for z-ordering
const PERSON_LEVEL: f32 = 2.0;

/// The distance below which a building applies its effect on a person.
const INTERACTION_DISTANCE: f32 = 20.0;

#[derive(Resource)]
pub struct UsedPersons {
    pub list: Vec<i32>,
}

#[derive(Resource, Debug)]
struct SpawnTimer(Timer);

#[derive(Resource, Debug)]
struct ScoreUpdateTimer(Timer);

#[derive(Component, Debug, Default, Clone)]
pub struct Person {
    pub id: i32,
    pub shelter: f32,
    pub hunger: f32,
    pub social: f32,
    pub entertained: f32,
    pub health: f32,
    pub sport: f32,
    pub creativity: f32,
    pub satisfaction: f32,
    // Idle movt direction
    pub movement_direction: MovementDir,
    pub movement_vector: Vec2,
    pub liked: Vec<i32>,
    pub disliked: Vec<i32>,
    pub interact: Interactable,
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
            .insert_resource(SpawnTimer(Timer::from_seconds(6.3, TimerMode::Repeating)))
            .insert_resource(ScoreUpdateTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )))
            .add_systems(OnEnter(GameState::Playing), spawn_first_person)
            .add_systems(
                Update,
                (
                    spawn_person,
                    decrease_scores,
                    increase_scores,
                    hitbox_follow,
                )
                    .run_if(in_state(GameState::Playing)),
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
            shelter: 10.0,
            hunger: 50.0,
            social: 75.0,
            entertained: 100.0,
            health: 100.0,
            sport: 100.0,
            creativity: 100.0,
            satisfaction: 50.0,
            movement_direction: MovementDir::PlusBoth,
            movement_vector: Vec2::ZERO,
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
    if spawn_timer.0.tick(time.delta()).just_finished() && used_ids.list.len() <= 2000 {
        let largest_id = used_ids.list.iter().max();
        let mut available_id = 0;
        if let Some(val) = largest_id {
            available_id = val + 1;
        }
        commands.spawn(PersonBundle {
            person: Person {
                id: available_id,
                shelter: 10.0,
                hunger: 50.0,
                social: 75.0,
                entertained: 100.0,
                health: 100.0,
                sport: 100.0,
                creativity: 100.0,
                satisfaction: 50.0,
                movement_direction: MovementDir::PlusBoth,
                movement_vector: Vec2::ZERO,
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

fn decrease_scores(
    mut persons_query: Query<&mut Person>,
    time: Res<Time>,
    mut update_timer: ResMut<ScoreUpdateTimer>,
) {
    if update_timer.0.tick(time.delta()).just_finished() {
        let mut rng = thread_rng();
        for mut person in &mut persons_query {
            person.shelter = clamp_score(person.shelter - 1.0);
            person.hunger = clamp_score(person.hunger - 0.75);

            let needs_sport = rng.gen_bool(0.7);
            let needs_creation = rng.gen_bool(0.68);
            let needs_entertainment = rng.gen_bool(0.45);
            let needs_social = rng.gen_bool(0.4);
            let health_incident = rng.gen_bool(0.005);

            if health_incident {
                person.health = clamp_score(person.health - 75.0);
            }
            if needs_entertainment {
                person.entertained = clamp_score(person.entertained - 1.0);
            }
            if needs_sport {
                person.sport = clamp_score(person.sport - 1.0);
            }
            if needs_creation {
                person.creativity = clamp_score(person.creativity - 1.0);
            }
            if needs_social {
                person.social = clamp_score(person.social - 1.0);
            }

            person.satisfaction = (person.shelter
                + person.hunger
                + person.health
                + person.entertained
                + person.sport
                + person.creativity
                + person.social)
                / 7.0;
        }
    }
}

fn increase_scores(
    mut persons_query: Query<(&mut Person, &Transform)>,
    buildings_query: Query<(&BuildingType, &Transform)>,
    time: Res<Time>,
) {
    for (mut person, p_transform) in &mut persons_query {
        for (b_type, b_transform) in &buildings_query {
            if p_transform.translation.distance(b_transform.translation) < INTERACTION_DISTANCE {
                match b_type {
                    BuildingType::House => person.shelter = 100.0,
                    BuildingType::Forum => {
                        person.social = clamp_score(person.social + 5.0 * time.delta_seconds());
                    }
                    BuildingType::Cinema => {
                        person.entertained =
                            clamp_score(person.entertained + 5.0 * time.delta_seconds());
                    }
                    BuildingType::Hospital => {
                        person.health = clamp_score(person.health + 10.0 * time.delta_seconds());
                    }
                    BuildingType::Pool => {
                        person.sport = clamp_score(person.sport + 10.0 * time.delta_seconds());
                    }
                    BuildingType::Restaurant => {
                        person.hunger = clamp_score(person.hunger + 50.0 * time.delta_seconds());
                    }
                    BuildingType::Creative => {
                        person.creativity =
                            clamp_score(person.creativity + 25.0 * time.delta_seconds());
                    }
                    _ => {}
                }
            }
        }
    }
}

fn clamp_score(val: f32) -> f32 {
    val.clamp(0.0, 100.0)
}

fn hitbox_follow(mut persons_query: Query<(&mut Person, &Transform)>) {
    for (mut person, transform) in &mut persons_query {
        person.interact.bottom_left.x = transform.translation.x - SPRITE_SIZE.x / 2.0;
        person.interact.bottom_left.y = transform.translation.y - SPRITE_SIZE.y / 2.0;
        person.interact.top_right.x = transform.translation.x + SPRITE_SIZE.x / 2.0;
        person.interact.top_right.y = transform.translation.y + SPRITE_SIZE.y / 2.0;
    }
}

fn cleanup_persons(
    mut commands: Commands,
    persons_query: Query<Entity, With<Person>>,
    mut used_ids: ResMut<UsedPersons>,
) {
    for entity in &persons_query {
        commands.entity(entity).despawn_recursive();
    }
    used_ids.list = vec![];
}

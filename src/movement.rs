use std::f32::consts::SQRT_2;

use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::{building::BuildingType, person::Person, states::GameState};

const MOVEMENT_SCALAR: f32 = 2.2;
const IDLE_INTERACT: f32 = 2.5;
const SOCIAL_INTERACT: f32 = 1.0;
const BUILDING_INTERACT: f32 = 6.0;

pub const HUNGER_THRESHOLD: f32 = 25.0;
pub const SHELTER_THRESHOLD: f32 = 10.0;
pub const SOCIAL_THRESHOLD: f32 = 25.0;
pub const ENTERT_THRESHOLD: f32 = 20.0;
pub const HEALTH_THRESHOLD: f32 = 30.0;
pub const SPORT_THRESHOLD: f32 = 20.0;
pub const CREAT_THRESHOLD: f32 = 20.0;

#[derive(Debug, Default, Clone, Copy)]
pub enum MovementDir {
    #[default]
    PlusX,
    PlusY,
    PlusBoth,
    MinusX,
    MinusY,
    MinusBoth,
}

impl Distribution<MovementDir> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MovementDir {
        match rng.gen_range(0..6) {
            0 => MovementDir::PlusX,
            1 => MovementDir::PlusY,
            2 => MovementDir::PlusBoth,
            3 => MovementDir::MinusX,
            4 => MovementDir::MinusY,
            5 => MovementDir::MinusBoth,
            _ => MovementDir::PlusBoth,
        }
    }
}

#[derive(Resource, Debug)]
struct IdleDirectionTimer(Timer);

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IdleDirectionTimer(Timer::from_seconds(
            3.0,
            TimerMode::Repeating,
        )))
        .add_systems(
            PreUpdate,
            reset_movement_vector.run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (move_idle_persons, social_movement, desire_movement)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            PostUpdate,
            resolve_movements.run_if(in_state(GameState::Playing)),
        );
    }
}

fn reset_movement_vector(mut person_query: Query<&mut Person>) {
    for mut person in &mut person_query {
        person.movement_vector = Vec2::ZERO;
    }
}

fn move_idle_persons(
    mut persons_query: Query<&mut Person>,
    time: Res<Time>,
    mut dir_timer: ResMut<IdleDirectionTimer>,
) {
    if dir_timer.0.tick(time.delta()).just_finished() {
        // change the directions at random
        for mut person in &mut persons_query {
            let new_dir: MovementDir = rand::random();
            person.movement_direction = new_dir;
        }
    }

    for mut person in &mut persons_query {
        match person.movement_direction {
            MovementDir::PlusX => {
                person.movement_vector.x += IDLE_INTERACT * 1.0;
            }
            MovementDir::PlusY => {
                person.movement_vector.y += IDLE_INTERACT * 1.0;
            }
            MovementDir::PlusBoth => {
                person.movement_vector.x += IDLE_INTERACT * 1.0 / SQRT_2;
                person.movement_vector.y += IDLE_INTERACT * 1.0 / SQRT_2;
            }
            MovementDir::MinusX => {
                person.movement_vector.x -= IDLE_INTERACT * 1.0;
            }
            MovementDir::MinusY => {
                person.movement_vector.y -= IDLE_INTERACT * 1.0;
            }
            MovementDir::MinusBoth => {
                person.movement_vector.x -= IDLE_INTERACT * 1.0 / SQRT_2;
                person.movement_vector.y -= IDLE_INTERACT * 1.0 / SQRT_2;
            }
        }
    }
}

fn move_relative_to(
    moved_person: &mut Person,
    moved_transform: &Transform,
    destination: &Transform,
    towards: bool,
    factor: f32,
) {
    let direction_mult = match towards {
        true => 1.0,
        false => -1.0,
    };
    moved_person.movement_vector += direction_mult
        * factor
        * (destination.translation - moved_transform.translation)
            .truncate()
            .clamp_length_max(5.0);
}

fn social_movement(mut persons_query: Query<(&mut Person, &Transform)>) {
    let mut combinations = persons_query.iter_combinations_mut();
    while let Some([(mut person_1, transform_1), (mut person_2, transform_2)]) =
        combinations.fetch_next()
    {
        if person_1.liked.contains(&person_2.id) {
            move_relative_to(
                &mut person_1,
                transform_1,
                transform_2,
                true,
                SOCIAL_INTERACT,
            );
        } else if person_1.disliked.contains(&person_2.id) {
            move_relative_to(
                &mut person_1,
                transform_1,
                transform_2,
                false,
                SOCIAL_INTERACT,
            );
        }
        if person_2.liked.contains(&person_1.id) {
            move_relative_to(
                &mut person_2,
                transform_2,
                transform_1,
                true,
                SOCIAL_INTERACT,
            );
        } else if person_2.disliked.contains(&person_1.id) {
            move_relative_to(
                &mut person_2,
                transform_2,
                transform_1,
                false,
                SOCIAL_INTERACT,
            );
        }
    }
}

fn desire_movement(
    mut persons_query: Query<(&mut Person, &Transform)>,
    buildings_query: Query<(&BuildingType, &Transform)>,
) {
    for (mut person, p_transform) in &mut persons_query {
        if person.hunger < HUNGER_THRESHOLD {
            move_relative_to(
                &mut person,
                p_transform,
                &get_closest_of_interest(p_transform, BuildingType::Restaurant, &buildings_query),
                true,
                4.0 * BUILDING_INTERACT,
            );
        }
        if person.shelter < SHELTER_THRESHOLD {
            move_relative_to(
                &mut person,
                p_transform,
                &get_closest_of_interest(p_transform, BuildingType::House, &buildings_query),
                true,
                0.5 * BUILDING_INTERACT,
            );
        }
        if person.health < HEALTH_THRESHOLD {
            move_relative_to(
                &mut person,
                p_transform,
                &get_closest_of_interest(p_transform, BuildingType::Hospital, &buildings_query),
                true,
                5.0 * BUILDING_INTERACT,
            );
        }
        if person.social < SOCIAL_THRESHOLD {
            move_relative_to(
                &mut person,
                p_transform,
                &get_closest_of_interest(p_transform, BuildingType::Forum, &buildings_query),
                true,
                3.0 * BUILDING_INTERACT,
            );
        }
        if person.creativity < CREAT_THRESHOLD {
            move_relative_to(
                &mut person,
                p_transform,
                &get_closest_of_interest(p_transform, BuildingType::Creative, &buildings_query),
                true,
                2.0 * BUILDING_INTERACT,
            );
        }
        if person.sport < SPORT_THRESHOLD {
            move_relative_to(
                &mut person,
                p_transform,
                &get_closest_of_interest(p_transform, BuildingType::Pool, &buildings_query),
                true,
                0.6 * BUILDING_INTERACT,
            );
        }
        if person.entertained < ENTERT_THRESHOLD {
            move_relative_to(
                &mut person,
                p_transform,
                &get_closest_of_interest(p_transform, BuildingType::Cinema, &buildings_query),
                true,
                BUILDING_INTERACT,
            );
        }
    }
}

fn get_closest_of_interest(
    p_transform: &Transform,
    desired_type: BuildingType,
    buildings_query: &Query<(&BuildingType, &Transform)>,
) -> Transform {
    let mut closest_of_interest = p_transform;
    let mut min_distance_of_interest = f32::MAX;
    for (b_type, b_transform) in buildings_query {
        if *b_type == desired_type {
            let newest_distance = p_transform.translation.distance(b_transform.translation);
            if newest_distance < min_distance_of_interest {
                min_distance_of_interest = newest_distance;
                closest_of_interest = b_transform;
            }
        }
    }
    *closest_of_interest
}

fn resolve_movements(mut person_query: Query<(&mut Person, &mut Transform)>, time: Res<Time>) {
    for (mut person, mut transform) in &mut person_query {
        if person.movement_vector.length_squared() == 0.0 {
            person.movement_vector += Vec2::new(0.01, 0.01);
        }
        person.movement_vector =
            person.movement_vector.clamp_length(0.25, 6.5) * MOVEMENT_SCALAR * time.delta_seconds();
        transform.translation.x += person.movement_vector.x;
        transform.translation.y += person.movement_vector.y;
    }
}

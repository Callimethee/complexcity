use std::f32::consts::SQRT_2;

use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::{
    person::{Person, PersonState},
    states::GameState,
};

const MOVEMENT_SCALAR: f32 = 2.0;
const INTERACTION_FORCE: f32 = 0.5;
const IDLE_FORCE: f32 = 4.5;

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
            (move_idle_persons, social_movement).run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            PostUpdate,
            resolve_movements.run_if(in_state(GameState::Playing)),
        );
    }
}

fn reset_movement_vector(mut person_query: Query<&mut Person>) {
    for mut person in &mut person_query {
        person.movement_vector = Vec3::ZERO;
    }
}

fn move_idle_persons(
    mut person_query: Query<&mut Person>,
    time: Res<Time>,
    mut dir_timer: ResMut<IdleDirectionTimer>,
) {
    if dir_timer.0.tick(time.delta()).just_finished() {
        // change the directions at random
        for mut person in &mut person_query {
            let new_dir: MovementDir = rand::random();
            person.movement_direction = new_dir;
        }
    }

    for mut person in &mut person_query {
        if matches!(person.state, PersonState::Idle) {
            match person.movement_direction {
                MovementDir::PlusX => {
                    person.movement_vector.x += IDLE_FORCE * 1.0;
                }
                MovementDir::PlusY => {
                    person.movement_vector.y += IDLE_FORCE * 1.0;
                }
                MovementDir::PlusBoth => {
                    person.movement_vector.x += IDLE_FORCE * 1.0 / SQRT_2;
                    person.movement_vector.y += IDLE_FORCE * 1.0 / SQRT_2;
                }
                MovementDir::MinusX => {
                    person.movement_vector.x -= IDLE_FORCE * 1.0;
                }
                MovementDir::MinusY => {
                    person.movement_vector.y -= IDLE_FORCE * 1.0;
                }
                MovementDir::MinusBoth => {
                    person.movement_vector.x -= IDLE_FORCE * 1.0 / SQRT_2;
                    person.movement_vector.y -= IDLE_FORCE * 1.0 / SQRT_2;
                }
            }
        }
    }
}

fn move_relative_to(
    moved_person: &mut Person,
    moved_transform: &Transform,
    destination: &Transform,
    towards: bool,
) {
    let direction_mult = match towards {
        true => 1.0,
        false => -1.0,
    };
    moved_person.movement_vector += direction_mult
        * INTERACTION_FORCE
        * (destination.translation - moved_transform.translation).clamp_length(0.0, 5.0);
}

fn social_movement(mut person_query: Query<(&mut Person, &Transform)>) {
    let mut combinations = person_query.iter_combinations_mut();
    while let Some([(mut person_1, transform_1), (mut person_2, transform_2)]) =
        combinations.fetch_next()
    {
        if person_1.liked.contains(&person_2.id) {
            move_relative_to(&mut person_1, transform_1, transform_2, true);
        } else if person_1.disliked.contains(&person_2.id) {
            move_relative_to(&mut person_1, transform_1, transform_2, false);
        }
        if person_2.liked.contains(&person_1.id) {
            move_relative_to(&mut person_2, transform_2, transform_1, true);
        } else if person_2.disliked.contains(&person_1.id) {
            move_relative_to(&mut person_2, transform_2, transform_1, false);
        }
    }
}

fn resolve_movements(mut person_query: Query<(&Person, &mut Transform)>, time: Res<Time>) {
    for (person, mut transform) in &mut person_query {
        transform.translation +=
            person.movement_vector.clamp_length_max(5.0) * MOVEMENT_SCALAR * time.delta_seconds();
    }
}

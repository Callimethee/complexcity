use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::{person::Person, states::GameState};

const MOVEMENT_FACTOR: f32 = 18.0;

#[derive(Debug)]
pub enum MovementDir {
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
            Update,
            move_idle_persons.run_if(in_state(GameState::Playing)),
        );
    }
}

fn move_idle_persons(
    mut person_query: Query<(&mut Person, &mut Transform)>,
    time: Res<Time>,
    mut dir_timer: ResMut<IdleDirectionTimer>,
) {
    if dir_timer.0.tick(time.delta()).just_finished() {
        // change the directions at random
        for (mut person, _transform) in &mut person_query {
            let new_dir: MovementDir = rand::random();
            person.movement_direction = new_dir;
        }
    }

    for (person, mut transform) in &mut person_query {
        match person.movement_direction {
            MovementDir::PlusX => {
                transform.translation.x += MOVEMENT_FACTOR * time.delta_seconds();
            }
            MovementDir::PlusY => {
                transform.translation.y += MOVEMENT_FACTOR * time.delta_seconds();
            }
            MovementDir::PlusBoth => {
                transform.translation.x += MOVEMENT_FACTOR * time.delta_seconds();
                transform.translation.y += MOVEMENT_FACTOR * time.delta_seconds();
            }
            MovementDir::MinusX => {
                transform.translation.x -= MOVEMENT_FACTOR * time.delta_seconds();
            }
            MovementDir::MinusY => {
                transform.translation.y -= MOVEMENT_FACTOR * time.delta_seconds();
            }
            MovementDir::MinusBoth => {
                transform.translation.x -= MOVEMENT_FACTOR * time.delta_seconds();
                transform.translation.y -= MOVEMENT_FACTOR * time.delta_seconds();
            }
        }
    }
}

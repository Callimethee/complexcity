use bevy::prelude::*;

use crate::{movement::MovementDir, states::GameState};

const SPRITE_SCALE: Vec3 = Vec3::new(3.0, 3.0, 0.0);

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

#[derive(Component, Debug)]
pub struct Person {
    pub id: i32,
    pub hunger: i32,
    pub state: PersonState,
    pub movement_direction: MovementDir,
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
            .add_systems(Update, (spawn_person).run_if(in_state(GameState::Playing)));
    }
}

fn spawn_person(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    used_ids: Res<UsedPersons>,
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
            },
            sprite: SpriteBundle {
                texture: asset_server.load("Adam_idle_16x16_front.png"),
                transform: Transform {
                    scale: SPRITE_SCALE,
                    ..default()
                },
                ..default()
            },
        });
    }
}

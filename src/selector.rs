use bevy::prelude::*;

use crate::{
    asset_loader::AssetHandles,
    debug::TEXT_SIZE,
    person::{Person, UsedPersons, SPRITE_SCALE},
    states::GameState,
};

const SELECTOR_LEVEL: f32 = 10.0;

#[derive(Component)]
pub struct PersonInfoText;

#[derive(Component, Debug)]
pub struct Selector {
    selected: i32,
}

#[derive(Bundle)]
struct SelectorBundle {
    selector: Selector,
    sprite: SpriteBundle,
}

pub struct SelectorPlugin;

impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (spawn_selector, spawn_person_info),
        )
        .add_systems(
            Update,
            (follow_selected_person, display_person_info, switch_selected)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            OnExit(GameState::Playing),
            (cleanup_selector, cleanup_info_text),
        );
    }
}

fn spawn_selector(mut commands: Commands, asset_handles: Res<AssetHandles>) {
    commands.spawn(SelectorBundle {
        selector: Selector { selected: 0 },
        sprite: SpriteBundle {
            texture: asset_handles.selector.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, SELECTOR_LEVEL),
                scale: SPRITE_SCALE,
                ..default()
            },
            ..default()
        },
    });
}

fn follow_selected_person(
    person_query: Query<(&Person, &Transform)>,
    mut selector_query: Query<(&Selector, &mut Transform), Without<Person>>,
) {
    let (selector, mut sel_transform) = selector_query.single_mut();

    for (person, transform) in &person_query {
        if person.id == selector.selected {
            sel_transform.translation.x = transform.translation.x;
            sel_transform.translation.y = transform.translation.y;
        }
    }
}

fn spawn_person_info(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Selected Person: ",
                TextStyle {
                    font_size: TEXT_SIZE,
                    ..default()
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font_size: TEXT_SIZE,
                    ..default()
                },
            ),
            TextSection::new(
                "\nLikes: ",
                TextStyle {
                    font_size: TEXT_SIZE,
                    ..default()
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font_size: TEXT_SIZE,
                    ..default()
                },
            ),
            TextSection::new(
                "\nDislikes: ",
                TextStyle {
                    font_size: TEXT_SIZE,
                    ..default()
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font_size: TEXT_SIZE,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: Val::VMax(0.5),
            top: Val::VMin(1.0),
            ..default()
        }),
        PersonInfoText,
    ));
}

fn display_person_info(
    mut text_query: Query<&mut Text, With<PersonInfoText>>,
    selector_query: Query<&Selector>,
    person_query: Query<&Person>,
) {
    let mut text = text_query.single_mut();
    let selector = selector_query.single();

    text.sections[1].value = format!("{}", selector.selected);
    for person in &person_query {
        if person.id == selector.selected {
            text.sections[3].value = format!("{:?}", person.liked);
            text.sections[5].value = format!("{:?}", person.disliked);
        }
    }
}

fn switch_selected(
    mut selector_query: Query<&mut Selector>,
    keys: Res<Input<KeyCode>>,
    used_ids: Res<UsedPersons>,
) {
    let mut selector = selector_query.single_mut();
    if keys.just_pressed(KeyCode::Right) {
        selector.selected = (selector.selected + 1).min(*used_ids.list.iter().max().unwrap())
    } else if keys.just_pressed(KeyCode::Left) {
        selector.selected = (selector.selected - 1).max(*used_ids.list.iter().min().unwrap())
    }
}

fn cleanup_selector(selector_query: Query<Entity, With<Selector>>, mut commands: Commands) {
    commands.entity(selector_query.single()).despawn_recursive();
}

fn cleanup_info_text(text_query: Query<Entity, With<PersonInfoText>>, mut commands: Commands) {
    commands.entity(text_query.single()).despawn_recursive();
}

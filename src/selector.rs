use bevy::prelude::*;

use crate::{
    asset_loader::AssetHandles,
    camera::CursorPosition,
    debug::TEXT_SIZE,
    drag::clicked_on,
    movement::{
        CREAT_THRESHOLD, ENTERT_THRESHOLD, HEALTH_THRESHOLD, HUNGER_THRESHOLD, SHELTER_THRESHOLD,
        SOCIAL_THRESHOLD, SPORT_THRESHOLD,
    },
    person::{Person, SPRITE_SCALE},
    states::GameState,
};

// The sentence corresponding to each problem
const SHELTER_SENTENCE: &str = "\nI need a comfy home...";
const HUNGER_SENTENCE: &str = "\nI could eat a horse!";
const SOCIAL_SENTENCE: &str = "\nI need friends...";
const ENTERT_SENTENCE: &str = "\nI'm bored.";
const HEALTH_SENTENCE: &str = "\nI don't feel so good...";
const SPORT_SENTENCE: &str = "\nI have energy to spare!";
const CREAT_SENTENCE: &str = "\nI feel like creating something today!";

// for z-ordering
const SELECTOR_LEVEL: f32 = 10.0;

#[derive(Resource, Debug, Default)]
struct Problems {
    shelter: bool,
    hunger: bool,
    social: bool,
    entertained: bool,
    health: bool,
    sport: bool,
    creativity: bool,
}

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
        app.init_resource::<Problems>()
            .add_systems(
                OnEnter(GameState::Playing),
                (spawn_selector, spawn_person_info),
            )
            .add_systems(
                Update,
                (follow_selected_person, update_person_info, switch_selected)
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

/// Make the selector follow the selected person
fn follow_selected_person(
    persons_query: Query<(&Person, &Transform)>,
    mut selector_query: Query<(&Selector, &mut Transform), Without<Person>>,
) {
    let (selector, mut sel_transform) = selector_query.single_mut();

    for (person, transform) in &persons_query {
        if person.id == selector.selected {
            sel_transform.translation.x = transform.translation.x;
            sel_transform.translation.y = transform.translation.y;
        }
    }
}

fn spawn_person_info(mut commands: Commands) {
    let text_style = TextStyle {
        font_size: TEXT_SIZE,
        ..default()
    };

    let problem_style = TextStyle {
        font_size: TEXT_SIZE - 2.0,
        color: Color::GOLD,
        ..default()
    };

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("Selected Person: ", text_style.clone()),
            TextSection::new("", text_style.clone()),
            TextSection::new("\nShelter: ", text_style.clone()),
            TextSection::new("", text_style.clone()),
            TextSection::new("\nHunger: ", text_style.clone()),
            TextSection::new("", text_style.clone()),
            TextSection::new("\nSocial: ", text_style.clone()),
            TextSection::new("", text_style.clone()),
            TextSection::new("\nEntertainment: ", text_style.clone()),
            TextSection::new("", text_style.clone()),
            TextSection::new("\nHealth: ", text_style.clone()),
            TextSection::new("", text_style.clone()),
            TextSection::new("\nSport: ", text_style.clone()),
            TextSection::new("", text_style.clone()),
            TextSection::new("\nCreativity: ", text_style.clone()),
            TextSection::new("", text_style.clone()),
            TextSection::new("\nSatisfaction: ", text_style.clone()),
            TextSection::new("", text_style.clone()),
            TextSection::new("\nCurrent Problems: ", text_style),
            TextSection::new("", problem_style),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::VMax(1.0),
            top: Val::VMin(1.0),
            ..default()
        }),
        PersonInfoText,
    ));
}

/// Update the display of all scores as well as the problems list
fn update_person_info(
    mut text_query: Query<&mut Text, With<PersonInfoText>>,
    selector_query: Query<&Selector>,
    persons_query: Query<&Person>,
    mut problems: ResMut<Problems>,
) {
    let mut text = text_query.single_mut();
    let selector = selector_query.single();

    text.sections[1].value = format!("{}", selector.selected);
    for person in &persons_query {
        if person.id == selector.selected {
            text.sections[3].value = format!("{:.0}", person.shelter);
            text.sections[5].value = format!("{:.0}", person.hunger);
            text.sections[7].value = format!("{:.0}", person.social);
            text.sections[9].value = format!("{:.0}", person.entertained);
            text.sections[11].value = format!("{:.0}", person.health);
            text.sections[13].value = format!("{:.0}", person.sport);
            text.sections[15].value = format!("{:.0}", person.creativity);
            text.sections[17].value = format!("{:.0}", person.satisfaction);

            if person.shelter < SHELTER_THRESHOLD && !problems.shelter {
                text.sections[19].value.push_str(SHELTER_SENTENCE);
                problems.shelter = true;
            } else if person.shelter >= SHELTER_THRESHOLD {
                text.sections[19].value = text.sections[19].value.replace(SHELTER_SENTENCE, "");
                problems.shelter = false;
            }

            if person.hunger < HUNGER_THRESHOLD && !problems.hunger {
                text.sections[19].value.push_str(HUNGER_SENTENCE);
                problems.hunger = true;
            } else if person.hunger >= HUNGER_THRESHOLD {
                text.sections[19].value = text.sections[19].value.replace(HUNGER_SENTENCE, "");
                problems.hunger = false;
            }

            if person.social < SOCIAL_THRESHOLD && !problems.social {
                text.sections[19].value.push_str(SOCIAL_SENTENCE);
                problems.social = true;
            } else if person.social >= SOCIAL_THRESHOLD {
                text.sections[19].value = text.sections[19].value.replace(SOCIAL_SENTENCE, "");
                problems.social = false;
            }

            if person.entertained < ENTERT_THRESHOLD && !problems.entertained {
                text.sections[19].value.push_str(ENTERT_SENTENCE);
                problems.entertained = true;
            } else if person.entertained >= ENTERT_THRESHOLD {
                text.sections[19].value = text.sections[19].value.replace(ENTERT_SENTENCE, "");
                problems.entertained = false;
            }

            if person.health < HEALTH_THRESHOLD && !problems.health {
                text.sections[19].value.push_str(HEALTH_SENTENCE);
                problems.health = true;
            } else if person.health >= HEALTH_THRESHOLD {
                text.sections[19].value = text.sections[19].value.replace(HEALTH_SENTENCE, "");
                problems.health = false;
            }

            if person.sport < SPORT_THRESHOLD && !problems.sport {
                text.sections[19].value.push_str(SPORT_SENTENCE);
                problems.sport = true;
            } else if person.sport >= SPORT_THRESHOLD {
                text.sections[19].value = text.sections[19].value.replace(SPORT_SENTENCE, "");
                problems.sport = false;
            }

            if person.creativity < CREAT_THRESHOLD && !problems.creativity {
                text.sections[19].value.push_str(CREAT_SENTENCE);
                problems.creativity = true;
            } else if person.creativity >= CREAT_THRESHOLD {
                text.sections[19].value = text.sections[19].value.replace(CREAT_SENTENCE, "");
                problems.creativity = false;
            }
        }
    }
}

fn switch_selected(
    mut selector_query: Query<&mut Selector>,
    persons_query: Query<&Person>,
    buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    cursor_pos: Res<CursorPosition>,
) {
    let mut selector = selector_query.single_mut();
    if buttons.just_pressed(MouseButton::Left) {
        for person in &persons_query {
            if clicked_on(&cursor_pos, &person.interact) {
                selector.selected = person.id;
                break;
            }
        }
    } else if keys.just_pressed(KeyCode::Tab) {
        // Select the least satisfied person
        let mut min_satis = 100.0;
        let mut min_id = 0;
        for person in &persons_query {
            if person.satisfaction < min_satis {
                min_satis = person.satisfaction;
                min_id = person.id;
            }
        }
        selector.selected = min_id;
    }
}

fn cleanup_selector(selector_query: Query<Entity, With<Selector>>, mut commands: Commands) {
    commands.entity(selector_query.single()).despawn_recursive();
}

fn cleanup_info_text(text_query: Query<Entity, With<PersonInfoText>>, mut commands: Commands) {
    commands.entity(text_query.single()).despawn_recursive();
}

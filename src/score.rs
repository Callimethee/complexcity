use bevy::prelude::*;

use crate::{
    debug::TEXT_SIZE,
    person::{Person, UsedPersons},
    states::GameState,
};

#[derive(Debug, Resource)]
pub struct Score(pub f32);

#[derive(Component, Debug)]
struct ScoreText;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0.0))
            .add_systems(OnEnter(GameState::Playing), spawn_score_display)
            .add_systems(
                PostUpdate,
                (update_score, update_score_display).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_score_text);
    }
}

fn spawn_score_display(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: TEXT_SIZE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: TEXT_SIZE,
                color: Color::GOLD,
                ..default()
            }),
        ]),
        ScoreText,
    ));
}

fn update_score(
    mut score: ResMut<Score>,
    persons_query: Query<&Person>,
    used_ids: Res<UsedPersons>,
) {
    score.0 = 0.0;
    for person in &persons_query {
        score.0 += person.satisfaction;
    }
    score.0 /= used_ids.list.len() as f32;
}

fn update_score_display(score: Res<Score>, mut text_query: Query<&mut Text, With<ScoreText>>) {
    text_query.single_mut().sections[1].value = format!("{:.0}", score.0);
}

fn cleanup_score_text(mut commands: Commands, text_query: Query<Entity, With<ScoreText>>) {
    for entity in &text_query {
        commands.entity(entity).despawn_recursive();
    }
}

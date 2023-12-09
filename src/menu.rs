use bevy::prelude::*;

use crate::states::GameState;

#[derive(Component, Debug)]
struct MenuUI;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(Update, click_buttons.run_if(in_state(GameState::MainMenu)))
            .add_systems(Update, back_to_menu.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::MainMenu), despawn_menu);
    }
}

fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            MenuUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(70.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::GOLD),
                    background_color: BackgroundColor(Color::GRAY),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::BLUE,
                            ..default()
                        },
                    ));
                });
        });
}

fn click_buttons(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        match *interaction {
            Interaction::Pressed => {
                game_state.set(GameState::Playing);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn despawn_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuUI>>) {
    for entity in &menu_query {
        commands.entity(entity).despawn_recursive();
    }
}

fn back_to_menu(keys: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::Back) {
        game_state.set(GameState::MainMenu);
    }
}

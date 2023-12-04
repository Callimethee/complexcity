use bevy::prelude::*;

use crate::states::GameState;

#[derive(Component, Debug)]
struct MainMenu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_menu)
            .add_systems(Update, click_buttons)
            .add_systems(OnExit(GameState::MainMenu), despawn_menu);
    }
}

fn spawn_menu(mut commands: Commands) {
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
            MainMenu,
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
                            color: Color::ALICE_BLUE,
                            ..default()
                        },
                    ));
                });
        });
}

fn click_buttons() {}

fn despawn_menu() {}

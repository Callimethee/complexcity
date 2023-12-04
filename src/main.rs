// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod camera;
mod debug;
mod menu;
mod movement;
mod person;
mod states;

use bevy::{asset::AssetMetaCheck, prelude::*};
use camera::Camera2dPlugin;
use debug::DebugPlugin;
use menu::MenuPlugin;
use movement::MovementPlugin;
use person::PersonPlugin;
use states::GameState;

fn main() {
    App::new()
        // Built-ins
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.3)))
        .insert_resource(Msaa::Off)
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Complexcity".to_string(),
                canvas: Some("#bevy".to_owned()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        // Custom
        .add_state::<GameState>()
        .add_plugins((Camera2dPlugin, MenuPlugin, PersonPlugin, MovementPlugin))
        .add_plugins(DebugPlugin)
        .run();
}

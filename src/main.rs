// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod asset_loader;
mod building;
mod camera;
mod debug;
mod drag;
mod ground;
mod menu;
mod movement;
mod person;
mod score;
mod selector;
mod states;

use asset_loader::AssetLoaderPlugin;
use bevy::{asset::AssetMetaCheck, prelude::*};
use building::BuildingPlugin;
use camera::Camera2dPlugin;
// use debug::DebugPlugin;
use drag::DragPlugin;
use ground::GroundPlugin;
use menu::MenuPlugin;
use movement::MovementPlugin;
use person::PersonPlugin;
use score::ScorePlugin;
use selector::SelectorPlugin;
use states::GameState;

fn main() {
    App::new()
        // Built-ins
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.3)))
        // No anti-aliasing for pixel art
        .insert_resource(Msaa::Off)
        // This avoid breaking web builds
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Complexcity".to_string(),
                canvas: Some("#bevy".to_owned()),
                fit_canvas_to_parent: true,
                // Let web shortcuts work (like f5)
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        // Custom
        .add_state::<GameState>()
        .add_plugins((
            AssetLoaderPlugin,
            BuildingPlugin,
            Camera2dPlugin,
            DragPlugin,
            GroundPlugin,
            MenuPlugin,
            MovementPlugin,
            PersonPlugin,
            ScorePlugin,
            SelectorPlugin,
        ))
        // .add_plugins(DebugPlugin)
        .run();
}

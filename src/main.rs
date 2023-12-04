mod camera;

use bevy::prelude::*;
use camera::Camera2dPlugin;

const BG_COLOR: Color = Color::rgb(0.1, 0.1, 0.2);

fn main() {
    App::new()
        // Built-ins
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BG_COLOR))
        // Custom Plugins
        .add_plugins(Camera2dPlugin)
        .run();
}

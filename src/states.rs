use bevy::prelude::*;

#[derive(States, Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    // Settings,
}

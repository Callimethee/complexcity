use bevy::prelude::*;

#[derive(States, Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum GameState {
    Loading,
    #[default]
    MainMenu,
    Playing,
    Settings,
}

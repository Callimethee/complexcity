use bevy::prelude::*;

use crate::{camera::CursorPosition, states::GameState};

/// Component for all Draggable entities.
#[derive(Debug, Default, Component)]
pub struct Draggable {
    pub interact: Interactable,
    pub being_dragged: bool,
}

/// Component for all click-interactable entities.
#[derive(Component, Default, Debug, Clone)]
pub struct Interactable {
    pub bottom_left: Vec2,
    pub top_right: Vec2,
}

pub struct DragPlugin;

impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, dragging_system.run_if(in_state(GameState::Playing)));
    }
}

fn dragging_system(
    mut draggables_query: Query<(&mut Draggable, &mut Transform)>,
    buttons: Res<Input<MouseButton>>,
    cursor_pos: Res<CursorPosition>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for (mut draggable, _transform) in &mut draggables_query {
            if clicked_on(&cursor_pos, &draggable.interact) {
                draggable.being_dragged = true
            }
        }
    }
    if buttons.pressed(MouseButton::Left) {
        for (draggable, mut transform) in &mut draggables_query {
            if draggable.being_dragged {
                transform.translation.x = cursor_pos.0.x;
                transform.translation.y = cursor_pos.0.y;
            }
        }
    }
    if buttons.just_released(MouseButton::Left) {
        for (mut draggable, _transform) in &mut draggables_query {
            draggable.being_dragged = false;
        }
    }
}

/// Returns `true` if the cursor is within the interactable's bounding box
pub fn clicked_on(cursor_pos: &Res<CursorPosition>, interactable: &Interactable) -> bool {
    cursor_pos.0.x > interactable.bottom_left.x
        && cursor_pos.0.y > interactable.bottom_left.y
        && cursor_pos.0.x < interactable.top_right.x
        && cursor_pos.0.y < interactable.top_right.y
}

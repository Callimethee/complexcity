use bevy::prelude::*;

use crate::{camera::CursorPosition, states::GameState};

#[derive(Debug, Default, Component)]
pub struct Draggable {
    pub bottom_left: Vec2,
    pub top_right: Vec2,
    pub being_dragged: bool,
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
    let mut offset = Vec2::ZERO;

    if buttons.just_pressed(MouseButton::Left) {
        for (mut draggable, _transform) in &mut draggables_query {
            if cursor_pos.0.x > draggable.bottom_left.x
                && cursor_pos.0.y > draggable.bottom_left.y
                && cursor_pos.0.x < draggable.top_right.x
                && cursor_pos.0.y < draggable.top_right.y
            {
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

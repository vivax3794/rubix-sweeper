use bevy::input::mouse::MouseMotion;

use super::cube::cube_data::FaceIdentifier;
use super::cube::turn::Turn;
use crate::cube::{Center, Cube, Face};
use crate::prelude::*;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (turn_input, pan_camera));
    }
}

pub fn turn_input(
    centers: Query<&FaceIdentifier, With<Center>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut click_events: EventReader<Pointer<Click>>,
    mut turn_events: EventWriter<Turn>,
) {
    for click in click_events.read() {
        if click.event.button == PointerButton::Middle {
            if let Ok(face) = centers.get(click.target) {
                turn_events.send(Turn {
                    face: *face,
                    flip_direction: keyboard.pressed(KeyCode::ShiftLeft),
                });
            }
        }
    }
}

const PAN_SCALE: f32 = 200.0;

fn pan_camera(
    mut query: Query<&mut Transform, With<Cube>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    let Ok(mut trans) = query.get_single_mut() else {
        return;
    };

    for motion in mouse_motion.read() {
        if mouse_buttons.pressed(MouseButton::Middle) {
            trans.rotate_y(motion.delta.x / PAN_SCALE);
            trans.rotate_z(-motion.delta.y / PAN_SCALE);
        }
    }
}

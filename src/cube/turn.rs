use bevy_tween::bevy_time_runner::{TimeRunner, TimeSpan};
use bevy_tween::tween::{TargetComponent, Tween};

use super::cube_data::{FaceIdentifier, PieceLocations, RelativeLocation};
use super::{Cube, Piece};
use crate::prelude::*;

#[derive(Event, Clone, Copy)]
pub struct Turn {
    pub face: FaceIdentifier,
    pub flip_direction: bool,
}

impl Turn {
    fn get_entities(self, locations: &PieceLocations) -> [Entity; 9] {
        locations.get_entities_on_face(self.face)
    }

    fn axis(self) -> Vec3 {
        let normal = self.face.normal();
        let mut axis = normal.abs();
        axis.y *= -1.0;
        axis
    }

    fn center(self) -> Vec3 {
        self.face.center().too_world()
    }

    fn update_location_data_based_on_turn(self, location_data: &mut PieceLocations) {
        let locations = self.face.piece_locations();
        let entities = self.get_entities(location_data);

        for index in 0..9 {
            let location = locations[index];
            let entity = entities[index];
            let new_location = if self.flip_direction {
                location.turn_clockwise()
            } else {
                location.turn_counterclockwise()
            };
            location_data.set(new_location.absolute(), entity);
        }
    }
}

impl RelativeLocation {
    const fn turn_counterclockwise(mut self) -> Self {
        let x = self.x;
        let y = self.y;

        self.x = -y;
        self.y = x;

        self
    }
    const fn turn_clockwise(mut self) -> Self {
        let x = self.x;
        let y = self.y;

        self.x = y;
        self.y = -x;

        self
    }
}

#[derive(Debug)]
pub struct RotateAround {
    start: Transform,
    axis: Vec3,
    center: Vec3,
    flip_direction: bool,
}

impl Interpolator for RotateAround {
    type Item = Transform;

    fn interpolate(&self, item: &mut Self::Item, value: f32) {
        let mut target = std::f32::consts::FRAC_PI_2; // 90 degrees
        if self.flip_direction {
            target *= -1.0;
        }
        let angle = (0.0).lerp(target, value);

        let rotation = Quat::from_axis_angle(self.axis, angle);

        *item = self.start;
        item.rotate_around(self.center, rotation);
    }
}

pub fn execute_turn(
    mut commands: Commands,
    mut cube_query: Query<&mut PieceLocations, With<Cube>>,
    mut piece_query: Query<
        (
            Entity,
            &mut Transform,
            Option<&Tween<TargetComponent, RotateAround>>,
        ),
        With<Piece>,
    >,
    mut turns: EventReader<Turn>,
) {
    let Ok(mut piece_locations) = cube_query.get_single_mut() else {
        return;
    };

    let amount = turns.len();
    if amount > 0 {
        skip_ahead_running_tweens(&mut commands, &mut piece_query);
    }

    for (index, turn) in turns.read().enumerate() {
        let entities = turn.get_entities(&piece_locations);
        let do_tween = index == amount - 1;

        for entity in entities {
            let Ok((_, piece_trans, _)) = piece_query.get_mut(entity) else {
                continue;
            };

            apply_rotation_or_rotate_entity(&mut commands, piece_trans, *turn, do_tween, entity);
        }

        turn.update_location_data_based_on_turn(&mut piece_locations);
    }
}

fn apply_rotation_or_rotate_entity(
    commands: &mut Commands,
    mut piece_trans: Mut<Transform>,
    turn: Turn,
    do_tween: bool,
    entity: Entity,
) {
    let axis = turn.axis();
    let center = turn.center();

    let duration = Duration::from_millis(500);
    let function = EaseFunction::ExponentialOut;
    let rotate_tween = RotateAround {
        start: *piece_trans,
        axis,
        center,
        flip_direction: turn.flip_direction,
    };

    if do_tween {
        let target = entity.into_target();
        commands.entity(entity).animation().insert_tween_here(
            duration,
            function,
            target.with(rotate_tween),
        );
    } else {
        rotate_tween.interpolate(&mut piece_trans, 1.0);
    }
}

fn skip_ahead_running_tweens(
    commands: &mut Commands,
    piece_query: &mut Query<
        (
            Entity,
            &mut Transform,
            Option<&Tween<TargetComponent, RotateAround>>,
        ),
        With<Piece>,
    >,
) {
    for (entity, mut current_trans, current_tween) in piece_query {
        if let Some(current_tween) = current_tween {
            current_tween
                .interpolator
                .interpolate(&mut current_trans, 1.0);
            commands
                .entity(entity)
                .remove::<Tween<TargetComponent, RotateAround>>();
        }
    }
}

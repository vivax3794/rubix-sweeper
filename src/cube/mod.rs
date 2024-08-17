use bevy_tween::component_tween_system;

use crate::prelude::*;

mod create;
pub mod cube_data;
pub mod turn;

#[derive(Component)]
pub struct Cube;

#[derive(Component)]
pub struct Piece;

#[derive(Component)]
pub struct Face;

#[derive(Component)]
pub struct Center;

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<turn::Turn>();
        app.add_systems(OnEnter(MainState::Playing), create::spawn_cube);
        app.add_systems(Update, turn::execute_turn);

        app.add_tween_systems(component_tween_system::<turn::RotateAround>());
    }
}

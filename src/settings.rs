use crate::prelude::*;

pub const CUBE_SIZE: f32 = 10.0;
pub const PIECE_SIZE: f32 = 3.0;
pub const PIECE_INSET: f32 = 1.0;

pub const CUBE_COLOR: Color = Color::BLACK;
pub const PIECE_FILL_COLOR: Color = Color::BLACK;
pub const SIDE_COLORS: [Color; 6] = [
    Color::linear_rgb(1.0, 1.0, 1.0),
    Color::linear_rgb(0.0, 155. / 255., 72. / 255.),
    Color::linear_rgb(185. / 255., 0.0, 0.0),
    Color::linear_rgb(0.0, 69. / 255., 173. / 255.),
    Color::linear_rgb(1.0, 89. / 255., 0.0),
    Color::linear_rgb(1.0, 213. / 255., 0.0),
];

pub const CAMERA_DISTANCE: f32 = 10.0;
pub const LIGHT_DISTANCE: f32 = 2.0;

pub const CAMERA_LOCATION: f32 = CUBE_SIZE + CAMERA_DISTANCE;
pub const LIGHT_LOCATION: f32 = CUBE_SIZE + LIGHT_DISTANCE;
pub const PIECE_SPACING: f32 = CUBE_SIZE / 2.0 - PIECE_INSET;

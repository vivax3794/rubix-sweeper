use super::cube_data::PieceLocations;
use super::{Cube, Face, Piece};
use crate::cube::cube_data::{FaceIdentifier, PieceLocation};
use crate::cube::Center;
use crate::prelude::*;
use crate::settings;

pub fn spawn_cube(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut location_data = PieceLocations::new();
    commands
        .spawn(SpatialBundle::default())
        .insert(Cube)
        .with_children(|commands| {
            location_data = spawn_pieces(commands, meshes, materials);
        })
        .insert(location_data);
}

#[allow(clippy::cast_precision_loss)]
fn spawn_pieces(
    commands: &mut ChildBuilder,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> PieceLocations {
    let mut locations = PieceLocations::new();

    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }

                let x = x as f32;
                let y = y as f32;
                let z = z as f32;
                let location = Vec3::new(x, y, z);

                let entity = spawn_piece(commands, location, &mut meshes, &mut materials);
                locations.set(location, entity);
            }
        }
    }

    locations
}

fn spawn_piece(
    commands: &mut ChildBuilder,
    side_normal: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let location = side_normal * settings::PIECE_SPACING;

    commands
        .spawn(SpatialBundle {
            transform: Transform::from_translation(location),
            ..default()
        })
        .insert(Piece)
        .with_children(|commands| {
            const BACKING_SIZE: f32 = settings::CUBE_SIZE / 3.0;
            const BACKING_INSET: f32 = settings::PIECE_INSET;

            commands.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(BACKING_SIZE, BACKING_SIZE, BACKING_SIZE)),
                material: materials.add(Color::BLACK),
                transform: Transform::from_translation(side_normal * -BACKING_INSET),
                ..default()
            });

            for face_index in 0..6 {
                let face = FaceIdentifier(face_index);

                let location = face.normal() * settings::PIECE_SIZE / 2.0;
                let outside = is_outside(face, side_normal);

                let color = if outside {
                    face.color()
                } else {
                    settings::PIECE_FILL_COLOR
                };

                let trans = Transform::from_translation(location)
                    .with_rotation(Quat::from_rotation_arc(Vec3::X, face.normal()));
                let mut builder = commands.spawn(PbrBundle {
                    mesh: meshes.add(Plane3d::new(
                        Vec3::X,
                        Vec2::new(settings::PIECE_SIZE, settings::PIECE_SIZE) / 2.0,
                    )),
                    material: materials.add(color),
                    transform: trans,
                    ..default()
                });
                if outside {
                    builder
                        .insert(Face)
                        .insert(face)
                        .insert(PickableBundle::default());

                    let center = [side_normal.x, side_normal.y, side_normal.z]
                        .into_iter()
                        .filter(|e| *e != 0.0)
                        .count()
                        == 1;

                    if center {
                        builder.insert(Center);
                    }
                }
            }
        })
        .id()
}

fn is_outside(face: FaceIdentifier, side_normal: Vec3) -> bool {
    let normal = face.normal();

    #[allow(clippy::float_cmp)] // We are always working with 1.0 and 0.0
    let outside = [
        (normal.x, side_normal.x),
        (normal.y, side_normal.y),
        (normal.z, side_normal.z),
    ]
    .into_iter()
    .any(|(n1, n2)| n1 == n2 && n1 != 0.0);
    outside
}

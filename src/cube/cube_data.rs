use crate::prelude::*;
use crate::settings;

/// Must be 0..6
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub struct FaceIdentifier(pub u8);

const FACE_NORMALS: [Vec3; 6] = [
    Vec3::Z,
    Vec3::X,
    Vec3::Y,
    Vec3::NEG_X,
    Vec3::NEG_Y,
    Vec3::NEG_Z,
];

impl FaceIdentifier {
    pub fn from_normal(normal: Vec3) -> Self {
        let normal = normal.normalize_or_zero();
        for (index, pot_normal) in FACE_NORMALS.iter().enumerate() {
            if normal == *pot_normal {
                return Self(index as u8);
            }
        }

        Self(0) // Fallback
    }

    pub const fn normal(self) -> Vec3 {
        FACE_NORMALS[self.0 as usize]
    }

    pub const fn center(self) -> PieceLocation {
        PieceLocation::from_vec(self.normal())
    }

    pub const fn color(self) -> Color {
        settings::SIDE_COLORS[self.0 as usize]
    }

    pub fn piece_locations(self) -> [RelativeLocation; 9] {
        let center = self.center();
        [
            center.relative(0, 0),
            center.relative(-1, -1),
            center.relative(-1, 0),
            center.relative(-1, 1),
            center.relative(0, -1),
            center.relative(0, 1),
            center.relative(1, -1),
            center.relative(1, 1),
            center.relative(1, 0),
        ]
    }
}

/// Cords must be -1,0,1.
/// 0,0,0 is not a valid location
#[derive(Component, Clone, Copy, Debug)]
pub struct PieceLocation {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl PieceLocation {
    pub const fn new(x: i8, y: i8, z: i8) -> Self {
        Self { x, y, z }
    }

    #[allow(clippy::cast_possible_truncation)] // We know the return value is -1.0,0.0,1.0
    pub const fn from_vec(pos: Vec3) -> Self {
        Self {
            x: (pos.x as i8).signum(),
            y: (pos.y as i8).signum(),
            z: (pos.z as i8).signum(),
        }
    }

    #[allow(clippy::cast_sign_loss)] // This should not happen
    const fn as_index(&self) -> usize {
        let x = if self.x >= 0 { self.x } else { self.x * -2 };
        let y = if self.y >= 0 { self.y } else { self.y * -2 };
        let z = if self.z >= 0 { self.z } else { self.z * -2 };

        let terny_num = x + y * 3 + z * 3 * 3;
        let index = terny_num - 1;

        index as usize
    }

    pub fn relative(self, x: i8, y: i8) -> RelativeLocation {
        RelativeLocation { center: self, x, y }
    }

    /// Given a vector containg just one non-zero element replace the zeros with the arguments
    /// (no ordering guarnties)
    const fn fill_in_position(self, a: i8, b: i8) -> PieceLocation {
        match [self.x, self.y, self.z] {
            [x, 0, 0] => Self::new(x, a, b),
            [0, y, 0] => Self::new(a, y, b),
            [0, 0, z] => Self::new(a, b, z),
            _ => self, // shouldnt happen
        }
    }

    pub fn too_world(self) -> Vec3 {
        let location = Vec3::new(self.x as f32, self.y as f32, self.z as f32);
        location * settings::PIECE_SPACING
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RelativeLocation {
    pub center: PieceLocation,
    pub x: i8,
    pub y: i8,
}

impl RelativeLocation {
    pub const fn absolute(self) -> PieceLocation {
        self.center.fill_in_position(self.x, self.y)
    }
}

impl From<Vec3> for PieceLocation {
    fn from(value: Vec3) -> Self {
        Self::from_vec(value)
    }
}

#[derive(Component, Clone)]
pub struct PieceLocations {
    entities: [Entity; 3 * 3 * 3 - 1],
}

impl PieceLocations {
    pub const fn new() -> Self {
        Self {
            entities: [Entity::from_raw(0); 3 * 3 * 3 - 1],
        }
    }

    pub fn set(&mut self, location: impl Into<PieceLocation>, entity: Entity) {
        self.entities[location.into().as_index()] = entity;
    }

    pub fn get(&self, location: impl Into<PieceLocation>) -> Entity {
        self.entities[location.into().as_index()]
    }

    pub fn get_entities_on_face(&self, face: FaceIdentifier) -> [Entity; 9] {
        face.piece_locations().map(|pos| self.get(pos.absolute()))
    }
}

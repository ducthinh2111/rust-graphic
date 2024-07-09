

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }

    pub fn scale(&self, scalar: f32) -> Self {
        Self {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
        }
    }

    pub(crate) fn magnitude(&self) -> f32 {
        let sum_square = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        sum_square.sqrt()
    }

    pub fn project_on(&self, vec: &Self) -> f32 {
        let dot_product = Self::dot_product(self, vec);
        dot_product / vec.magnitude()
    }

    fn dot_product(vec_1: &Self, vec_2: &Self) -> f32 {
        vec_1.x * vec_2.x + vec_1.y * vec_2.y + vec_1.z * vec_2.z
    }

    pub fn from_two_position(position_vec_1: &Self, position_vec_2: &Self) -> Self {
        Self {
            x: position_vec_2.x - position_vec_1.x,
            y: position_vec_2.y - position_vec_1.y,
            z: position_vec_2.z - position_vec_1.z
        }
    }

    pub fn add(vec_1: &Self, vec_2: &Self) -> Self {
        Self {
            x: vec_1.x + vec_2.x,
            y: vec_1.y + vec_2.y,
            z: vec_1.z + vec_2.z
        }
    }
}
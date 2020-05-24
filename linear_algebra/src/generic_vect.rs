use std::ops;

pub trait Field: ops::Add<Output = Self> + ops::Mul<Output = Self> + Sized {
    fn get_neutral() -> Self;
    fn get_inverse(val: Self) -> Self;
    fn get_symmetric(val: Self) -> Self;
}

pub trait VectorSpace<F: Field>: ops::Add<Output = Self> + Sized {
    fn scalar_mul(&mut self, scalar: F);
}

pub struct Vector {
    dim: u32,
    vals: Vec<f32>,
}

impl Vector {
    pub fn new(dim: u32, vals: Vec<f32>) -> Vector {
        Vector { dim, vals }
    }
}

impl Field for f32 {
    fn get_neutral() -> f32 {
        1.0
    }
    fn get_inverse(val: f32) -> f32 {
        1.0 / val
    }
    fn get_symmetric(val: f32) -> f32 {
        -val
    }
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        let mut new_vector: Vec<f32> = Vec::new();
        for (v1, v2) in self.vals.iter().zip(other.vals) {
            new_vector.push(v1 + v2);
        }
        Vector::new(self.dim, new_vector)
    }
}

impl VectorSpace<f32> for Vector {
    fn scalar_mul(&mut self, scalar: f32) {
        for val in &mut self.vals {
            *val *= scalar;
        }
    }
}

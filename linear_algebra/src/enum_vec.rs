use std::ops::{Add, Mul};

pub enum Vect<T> {
    TwoDim(T, T),
    ThreeDim(T, T, T),
}

impl<T> Vect<T>
where
    T: Field,
{
    pub fn dot(u: Vect<T>, v: Vect<T>) -> Option<T> {
        match (u, v) {
            (Vect::TwoDim(x1, y1), Vect::TwoDim(x2, y2)) => Some(x1 * x2 + y1 * y2),
            (Vect::ThreeDim(x1, y1, z1), Vect::ThreeDim(x2, y2, z2)) => {
                Some(x1 * x2 + y1 * y2 + z1 * z2)
            }
            (_, _) => None,
        }
    }

    pub fn cross(u: Vect<T>, v: Vect<T>) -> Option<Vect<T>> {
        match (u, v) {
            (Vect::TwoDim(_, _), _) => None,
            (_, Vect::TwoDim(_, _)) => None,
            (Vect::ThreeDim(x1, y1, z1), Vect::ThreeDim(x2, y2, z2)) => Some(Vect::ThreeDim(
                y1 * z2 - z1 * y2,
                z1 * x2 - x1 * z2,
                x1 * y2 - y1 * x2,
            )),
        }
    }

    pub fn scale(&mut self, r: i32) {
        match self {
            Vect::TwoDim(x, y) => {
                *x = *x * r;
                *y *= r
            }
            Vect::ThreeDim(x, y, z) => {
                *x *= r;
                *y *= r;
                *z *= r;
            }
        }
    }

    pub fn show(&self) {
        match self {
            Vect::TwoDim(x, y) => println!("< {0}, {1} >", x, y),
            Vect::ThreeDim(x, y, z) => println!("< {0}, {1}, {2} >", x, y, z),
        }
    }
}

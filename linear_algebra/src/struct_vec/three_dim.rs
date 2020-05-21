pub struct Vect {
    pub x : i32,
    pub y : i32,
    pub z : i32,
}

impl Vect {
    pub fn build(x : i32, y : i32, z : i32) -> Vect {
        Vect { x, y, z }
    }
}

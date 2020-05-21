pub enum Vec {
    TwoDim(i32, i32),
    ThreeDim(i32, i32, i32),
}

impl Vec {

    pub fn dot(u : &Vec, v : &Vec) -> Option<i32> {
        match (u, v) {
            (Vec::TwoDim(x1, y1), Vec::TwoDim(x2, y2)) => Some(x1 * x2 + y1 * y2),
            (Vec::ThreeDim(x1, y1, z1), Vec::ThreeDim(x2, y2, z2)) => Some(x1 * x2 + y1 * y2 + z1 * z2),
            (_, _) => None,
        }
    }

    pub fn cross(u : &Vec, v : &Vec) -> Option<Vec> {
        match (u, v) {
            (Vec::TwoDim(_, _), _) => None,
            (_, Vec::TwoDim(_, _)) => None,
            (Vec::ThreeDim(x1, y1, z1), Vec::ThreeDim(x2, y2, z2)) => Some(Vec::ThreeDim(y1 * z2 - z1 * y2,
                                                                                         z1 * x2 - x1 * z2,
                                                                                         x1 * y2 - y1 * x2)),
        }
    }

    pub fn scale(& mut self, r : i32) {
        match self {
            Vec::TwoDim(x, y) => {
                *x *= r;
                *y *= r
            },
            Vec::ThreeDim(x, y, z) => {
                *x *= r;
                *y *= r;
                *z *= r;
            },
        }
    }

    pub fn show(&self) {
        match self {
            Vec::TwoDim(x, y) => println!("< {0}, {1} >", x, y),
            Vec::ThreeDim(x, y, z) => println!("< {0}, {1}, {2} >", x, y, z),
        }
    }
}

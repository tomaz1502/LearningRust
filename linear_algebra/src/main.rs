#[allow(dead_code)]

enum Vec {
    TwoDim(i32, i32),
    ThreeDim(i32, i32, i32),
}

impl Vec {
    fn dot(u : Vec, v : Vec) -> Option<i32> {
        match (u, v) {
            (Vec::TwoDim(x1, y1), Vec::TwoDim(x2, y2)) => Some(x1 * x2 + y1 * y2),
            (Vec::ThreeDim(x1, y1, z1), Vec::ThreeDim(x2, y2, z2)) => Some(x1 * x2 + y1 * y2 + z1 * z2),
            (_, _) => None,
        }
    }

}


fn main() {
    let v = Vec::TwoDim(0, 1);
    let u = Vec::TwoDim(1, 0);
    match Vec::dot(u, v) {
        Some(x) => println!("{}", x),
        None    => println!("erro"),
    }
}

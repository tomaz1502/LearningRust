struct Vec2 {
    x : i32,
    y : i32,
}

struct Vec3 {
    x : i32,
    y : i32,
    z : i32,
}

impl Vec2 {
    fn build(x : i32, y : i32) -> Vec2 {
        Vec2 {
            x,
            y,
        }
    }
    
    fn show(&self) {
        println!("[{0}, {1}]", self.x, self.y);
    }
    
    fn dot(u : &Vec2, v : &Vec2) -> i32 {
        u.x * v.x + u.y * v.y
    }

    fn scale(& mut self, r : i32) {
        self.x *= r;
        self.y *= r;
    }
}

impl Vec3 {
    fn build(x : i32, y : i32, z : i32) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }

    fn show(&self) {
        println!("[{0}, {1}, {2}]", self.x, self.y, self.z);
    }

    fn cross(v : &Vec3, u : &Vec3) -> Vec3 {
        Vec3::build(v.y * u.z - v.z * u.y,
                    v.z * u.x - v.x * u.z,
                    v.x * u.y - v.y * u.x)
    }
}

fn main() {

    let k3 = Vec3::build(0,0,1);
    let j3 = Vec3::build(0,1,0);
    Vec3::cross(&j3, &k3).show();

    let mut i2 = Vec2::build(1,0);
    let j2 = Vec2::build(0,1);

    let dot_val = Vec2::dot(&i2, &j2);
    println!("{}", dot_val);

    i2.scale(2);
    i2.show();

}

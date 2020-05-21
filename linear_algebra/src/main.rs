mod enum_vec;

use enum_vec::Vec;

fn main() {
    let foo : Vec = Vec::TwoDim(2, 3);
    let oof : i32 = Vec::dot(&foo, &Vec::TwoDim(3, 2)).unwrap();
    let bar : Vec = Vec::ThreeDim(0, 0, 1);
    let baz : Vec = Vec::ThreeDim(0, 1, 0);
    
    let mut boom = Vec::cross(&bar, &baz).unwrap();
    boom.scale(10);

    println!("{}", oof);
    foo.show();
    bar.show();
    baz.show();
    boom.show();


}


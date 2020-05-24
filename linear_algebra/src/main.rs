mod enum_vec;
mod generic_vect;

use enum_vec::Vect;

fn main() {
    let foo: Vect = Vect::TwoDim(2, 3);
    let oof: i32 = Vect::dot(&foo, &Vect::TwoDim(3, 2)).unwrap();
    let bar: Vect = Vect::ThreeDim(0, 0, 1);
    let baz: Vect = Vect::ThreeDim(0, 1, 0);

    let mut boom = Vect::cross(&bar, &baz).unwrap();
    boom.scale(10);

    println!("{}", oof);
    foo.show();
    bar.show();
    baz.show();
    boom.show();

    let v = generic_vect::Vector::new(2, vec![2.0, 3.0]);
}

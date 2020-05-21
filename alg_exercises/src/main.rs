mod book;
mod code_wars;

use book::*;
use code_wars::*;

fn main() {
    let s = pig::conv_to_pig(&"tomaz".to_string());
    println!("{}", s);


    let t = stats::meanmedmod(&vec![1,2,3]);
    println!("{:?}", t);


    let u = change::change("tomaz");
    println!("{}", u);

    let v = ordered_count::ordered_count("tomaz");
    println!("{:?}", v);
}

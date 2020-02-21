use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {0}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Input Failed!");

        let guess = guess.trim();
        let guess : i32 = match guess.parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("You Guessed: {0}", guess);

        match guess.cmp(&secret_number)
        {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            },
        }
    }
}

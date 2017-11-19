extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(0, 101);
    println!("The secret to be guessed is {}", secret);
    println!("Make a guess!");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed {}", guess);
}

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(0, 101);
    println!("The secret to be guessed is {}", secret);
    println!("Make a guess!");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse()
        .expect("Please enter a number");
    println!("You guessed {}", guess);

    match guess.cmp(&secret){
        Ordering::Less    => println!("Too small"),
        Ordering::Greater => println!("Too large"),
        Ordering::Equal   => println!("Finished!"),
    }
}

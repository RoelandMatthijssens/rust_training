extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(0, 101);

    loop {
        println!("Make a guess!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 =match guess.trim().parse() {
            Ok(x)   => x,
            Err(_)  => {
                println!("Please enter a number");
                continue;
            },
        };
        println!("You guessed {}", guess);

        match guess.cmp(&secret){
            Ordering::Less    => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal   => {
                println!("Finished!");
                break;
            }
        };
        println!();
    }
}

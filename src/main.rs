extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");
    let rand_num = rand::thread_rng().gen_range(1,101);
    // println!("The secret number is {}", rand_num);
    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess);
        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}

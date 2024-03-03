/*
    Write a program that types from the keyboard,
    and handle the input to see the expected output.
    I will write a function that create a random number -> input from the keyboard
    check the input  <, >, = the random number. If = then break the loop.
*/

// Skip the warning by using the following:
#![allow(non_snake_case)]

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    guessing_game();
}

fn guessing_game() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Read the input from the keyboard
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the input to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

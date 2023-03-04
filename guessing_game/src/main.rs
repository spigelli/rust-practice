use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let secret_number = rand::thread_rng()
        .gen_range(1..11);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    // Now we need to cast the string to an integer
    // We can do this with the parse method
    let guess: u32 = match guess
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                // In the case that there was an error,
                // set the guess to 0
                0
            },
        };

    let did_win: bool;

    // Now we can compare the guess to the secret number
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            did_win = false;
            println!("Too small!")
        }
        Ordering::Greater => {
            did_win = false;
            println!("Too big!")
        }
        Ordering::Equal => {
            did_win = true;
            println!("You win!")
        }
    }

    if !did_win {
        print!("You lost! Boohoo! ");
        println!("The secret number was: {secret_number}");
    } else {
        println!("You won! Yay!")
    }
}

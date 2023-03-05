use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("=== Guess the number! ===");

    let secret_number = rand::thread_rng()
        .gen_range(1..11);

    run_round(secret_number);
}

fn run_round(secret_number: u32) {
    let guess = get_guess();

    // Now we can compare the guess to the secret number
    let did_win = match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!");
            false
        }
        Ordering::Greater => {
            println!("Too big!");
            false
        }
        Ordering::Equal => {
            println!("You win!");
            true
        }
    };

    if !did_win {
        println!("You lost! Boohoo! ");
        let should_retry = get_retry();
        if should_retry {
            run_round(secret_number);
        } else {
            println!("Sucks for you loser, the number was {}!", secret_number);
        }
    } else {
        println!("You won! Yay!");
    }
}

fn get_retry() -> bool {
    println!("Try again? (y/n)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return match input.trim() {
        "y" => true,
        "n" => false,
        _ => {
            println!("Please type y or n!");
            get_retry()
        },
    }
}

fn get_guess() -> u32 {
    println!("Please input your guess.");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You guessed: {}", input.trim());

    // Now we need to cast the string to an integer
    // We can do this with the parse method
    match input
        .trim()
        .parse::<u32>() {
            Ok(num) => {
                // Set the guess to the number and convert num from u32 to &mut u32
                num
            },
            Err(_) => {
                println!("Please type a number!");
                // Clear the guess
                get_guess()
            },
        }
}

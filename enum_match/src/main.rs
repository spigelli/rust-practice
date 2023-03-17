fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
}

fn die() {
    let dice_roll = 9;
    match dice_roll {
        1 => println!("You rolled a 1"),
        2 => println!("You rolled a 2"),
        3 => println!("You rolled a 3"),
        4 => println!("You rolled a 4"),
        5 => println!("You rolled a 5"),
        6 => println!("You rolled a 6"),
        // Since the value is not used in the match, we can use the _ placeholder
        _ => println!("What just happened?"),
    }
}
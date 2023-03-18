fn main() {
    println!("Hello, world!");
}


fn configmax() {
    let config_max = Some(3u8);
    #[allow(clippy::single_match)]
    match config_max {
        Some(max) => println!("Max is {}", max),
        _ => (),
    }
    
    if let Some(max) = config_max {
        println!("Max is {}", max);
    }
}

enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn coin() {
    let coin = Coin::Quarter(String::from("Alabama"));
    #[allow(unused_variables)]  
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}

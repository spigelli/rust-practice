fn main() {
    println!("Hello, world!");

    another_function(23);
    println!("Statements: {}", statements());
}
// Statements are void functions
// Expressions evaluate to a value

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is {}", x)
}

fn statements() -> i32 {
    // You can create an arbitrary expression using curlies
    let result = {
        let x = 1;
        let y = 2;
        y+x
    };
    result
}

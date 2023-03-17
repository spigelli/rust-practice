fn tuples() {
    // Declare a tuple that contains an integer, a floating-point number, and a boolean
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Assign by matching
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}

fn main() {
    println!("Hello, world!");
    tuples();
}

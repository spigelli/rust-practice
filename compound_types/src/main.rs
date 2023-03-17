fn tuples() {
    // Declare a tuple that contains an integer, a floating-point number, and a boolean
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Assign by matching
    let (x, y, z) = tup;
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("The value of z is {}", z);

    println!("The value of tup.0 is {}", tup.0);
}

fn arrays() {
    // Arrays must be of the same type
    // Arrays are stack allocated
    // Arrays should be fixed length
    
    let a = [1, 2, 3, 4, 5];

    // Arrays can have a default value
    let b = [0; 10]; // 10 zeros
    for i in 0..10 {
        println!("b[{}] = {}", i, b[i])
    }
}


fn main() {
    println!("Hello, world!");
    tuples();
    println!("\n");
    arrays();
}

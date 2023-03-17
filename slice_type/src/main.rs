fn main() {
    println!("Hello, world!");
}

// Maintains a reference to the data without taking ownership
fn first_word(s: &str) -> &str {
    // create an array of bytes from the string reference
    let bytes = s.as_bytes();
    //   v-- this is destructuring the return of enumerate() 
    for (i, &item) in bytes.iter().enumerate() {
        // if the item is equal to the byte representation of a space
        // return the index of the item
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn slice() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    
}
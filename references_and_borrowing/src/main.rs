fn main() {
    println!("Hello, world!");
}

fn referenceString() {
    let s1 = String::from("hello");
    let length = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, length);
}

fn calculate_length(s: &String) -> usize {
    // s is a REFERENCE to a String and immutable
    s.len()
}

fn mutableReference() {
    let mut s = String::from("hello");
    change(&mut s);
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    println!("{}", s);
}

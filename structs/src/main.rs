struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "User {{ active: {}, username: {}, email: {}, sign_in_count: {} }}", self.active, self.username, self.email, self.sign_in_count)
    }
}

fn main() {
    println!("Hello, world!");
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("x@x.com"),
        sign_in_count: 1,
    };
    println!("user1: {}", user1);
    let user2 = build_user(
        String::from("email"),
        String::from("username"),
    );
    println!("user2: {}", user2);
    let user3 = user_from_user_and_consume(
        String::from("user3"),
        user2,
    );
    println!("user3: {}", user3);
    // This would error now
    // println!("user2: {}", user2);

    // user1 is an Instance of the User struct
    
}

fn build_user(
    email: String,
    username: String,
) -> User {
    User {
        email, // shorthand just like in JS
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn user_from_user_and_consume(
    name: String,
    user: User,
) -> User {
    // user will be unusable because email was moved
    User {
        username: name,
        ..user // spread operator
    }
}

// This is cool
struct RGB(u8, u8, u8);
fn tuple_struct() {
    let black = RGB(0, 0, 0);
    let white = RGB(255, 255, 255);
}



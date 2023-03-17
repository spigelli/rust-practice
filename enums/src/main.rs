fn main() {
    println!("Hello, world!");
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;
    let myIp = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("0.0.0.0"),
    };
    
    Log::Info.stdout("This is an info message");
    Log::Info.stdout("This is an info message");
    Log::Error.stdout("This is an info message");
}

enum IpAddressKind {
    V4,
    V6,
}

struct IpAddress{
    kind: IpAddressKind,
    address: String,
}

enum Log {
    Info,
    Warning,
    Error,
}

impl Log {
    fn stdout(&self, msg: &str) {
        match self {
            // Info should print in blue
            Log::Info => println!("\x1b[34m{}\x1b[0m", msg),
            // Warning should print in yellow
            Log::Warning => println!("\x1b[33m{}\x1b[0m", msg),
            // Error should print in red
            Log::Error => println!("\x1b[31m{}\x1b[0m", msg),
        }
    }
}
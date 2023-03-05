/// Alias log to println
macro_rules! log {
    ($($arg:tt)*) => (println!($($arg)*));
}

fn immutable_variables() {
    // Immutable variable
    let immutable_variable = 6;
    log!("The value of immutable_variable is: {}", immutable_variable);
    log!("If we were to write immutable_variable = 7, we would get an error");
}

fn mutable_variables() {
    // Mutable variable
    log!("let mut mutable_variable = 6;");
    let mut mutable_variable = 6;

    log!("The value of mutable_variable is: {}", mutable_variable);

    log!("mutable_variable = 7;");
    mutable_variable = 7;

    log!("The value of mutable_variable is: {}", mutable_variable);
}

const FILE_WISE_GLOBAL_CONSTANT: u32 = 100_000;
fn constants() {
    // Constants
    log!("Constants can be declared with the const keyword");
    log!("e.g. const MAX_POINTS: u32 = 100_000;");

    log!("{}", [
        "const EXAMPLE_CONSTANT: String = ",
        "\"This is an example constant\";",
    ].concat());

    const EXAMPLE_CONSTANT: &str = "This is an example constant";

    log!("The value of EXAMPLE_CONSTANT is: {}", EXAMPLE_CONSTANT);

    log!("");
    log!("Constants must be annotated with a type");

    log!("const FILE_WISE_GLOBAL_CONSTANT: u32 = 100_000;");
    log!(
        "The value of FILE_WISE_GLOBAL_CONSTANT is: {}",
        FILE_WISE_GLOBAL_CONSTANT,
    );
}

fn shadowning() {
    let x = 5;
    let x = x + 1;

    #[allow(unused_variables)]
    let x = x * 2;
}

fn arbitrary_scoping() {
    // You can just arbitrarily create scopes
    // this is sick
    const CONSTANT_VAR: i32 = 1;
    
    #[allow(unused_assignments)]
    let mut mutable_var = 1;
    mutable_var = 2;

    log!("mutable_var in outer scope is: {}", mutable_var);
    log!("CONSTANT_VAR in outer scope is: {}", CONSTANT_VAR);

    {
        mutable_var = 3;
        log!("mutable_var in inner scope is: {}", mutable_var);

        // IMPORTANT:
        // If a constant is shadowed in an inner scope,
        // the inner scope's constant is used no matter when it is accessed
        log!("CONSTANT_VAR in inner scope is: {}", CONSTANT_VAR);

        const CONSTANT_VAR: i32 = 2;
        log!("CONSTANT_VAR in inner scope is: {}", CONSTANT_VAR);
    }

    log!("mutable_var in outer scope is: {}", mutable_var);
    log!("CONSTANT_VAR in outer scope is: {}", CONSTANT_VAR);
}

fn main() {
    immutable_variables();
    mutable_variables();
    constants();
    shadowning();
    arbitrary_scoping();
}
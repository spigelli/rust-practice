/// Alias log to println
macro_rules! log {
    ($($arg:tt)*) => (println!($($arg)*));
}

/// Prints a table
/// - The first row is a top border
/// - The second row is the headers, delimited by a pipe, and padded to 
///   pad_width
/// - The third row is a separator, delimited by a pipe
/// - The next section of rows are the data, delimited by a pipe, and padded to
///   pad_width
/// - The last row is a bottom border
/// 
/// Corners should be added to all borders e.g. ┌┐└┘
/// 
/// Vertical pipes should be added to both sides of all rows except the top
/// and bottom borders
/// Rows should be separated by a newline
fn print_table(headers: Vec<&str>, data: Vec<Vec<&str>>) {
    let num_columns = headers.len();
    let mut table = String::new();
        // The maximum length of all cells
    let max_width_data = data
        .iter()
        .flatten()
        .map(|cell| cell.len())
        .max()
        .unwrap();
    let max_width_headers = headers
        .iter()
        .map(|header| header.len())
        .max()
        .unwrap();
    let max_width = max_width_data.max(max_width_headers);
    let pad_width = max_width_data.max(max_width + 1);

    // Top border
    table.push_str("┌");
    for _ in 0..num_columns {
        table.push_str(&"─".repeat(pad_width));
        table.push_str("┬");
    }
    table.pop(); // Remove the last ┬
    table.push_str("┐\n");

    // Headers
    table.push_str("│");
    for header in headers {
        table.push_str(&header.pad_to_width(pad_width));
        table.push_str("│");
    }
    table.push_str("\n");

    // Separator
    table.push_str("├");
    for _ in 0..num_columns {
        table.push_str(&"─".repeat(pad_width));
        table.push_str("┼");
    }
    table.pop(); // Remove the last ┼
    table.push_str("┤\n");

    // Data
    for row in data {
        table.push_str("│");
        for cell in row {
            table.push_str(&cell.pad_to_width(pad_width));
            table.push_str("│");
        }
        table.push_str("\n");
    }

    // Bottom border
    table.push_str("└");
    for _ in 0..num_columns {
        table.push_str(&"─".repeat(pad_width));
        table.push_str("┴");
    }
    table.pop(); // Remove the last ┴
    table.push_str("┘\n");

    log!("{}", table);
}

/// Pads a string to a specific width
/// 
/// If the string is longer than the width, it will be truncated
/// 
/// If the string is shorter than the width, it will be padded with spaces
trait PadToWidth {
    fn pad_to_width(&self, width: usize) -> String;
}

impl PadToWidth for str {
    fn pad_to_width(&self, width: usize) -> String {
        let mut string = self.to_string();
        if string.len() > width {
            string.truncate(width);
        } else {
            string.push_str(&" ".repeat(width - string.len()));
        }
        string
    }
}

fn print_integer_types() {
    let headers = vec!["Length", "Signed", "Unsigned"];
    let data = vec![
        vec!["8-bit", "i8", "u8"],
        vec!["16-bit", "i16", "u16"],
        vec!["32-bit", "i32", "u32"],
        vec!["64-bit", "i64", "u64"],
        vec!["128-bit", "i128", "u128"],
        vec!["arch", "isize", "usize"],
    ];
    print_table(headers, data);
}

fn integer_types() {
    log!("=========Integer Types=========");
    log!("Integer types are used to represent whole numbers");
    log!("There are signed and unsigned versions of each integer type");
    log!("{}", [
        "Aditionally, there are types that are the same size as the", 
        "architecture they are running on (arch: isize, usize)",
    ].join(" "));
    log!("{}", [
        "These are 64 bits and 32 bits for 64-bit and 32-bit architectures",
        "respectively",
    ].join(" "));
    log!("");
    log!("         Integer Types");
    print_integer_types();
}

fn floats() {
    log!("=========Float Types=========");
    log!("Float types are used to represent fractional numbers");
    log!("There are signed and unsigned versions of each float type");
    log!("{}", [
        "Aditionally, there are types that are the same size as the", 
        "architecture they are running on (arch: f32, f64)",
    ].join(" "));
    log!("{}", [
        "These are 64 bits and 32 bits for 64-bit and 32-bit architectures",
        "respectively",
    ].join(" "));
    log!("");
    log!("         Float Types");
    print_floats();
}

fn print_floats() {
    let headers = vec!["Length", "Signed", "Unsigned"];
    let data = vec![
        vec!["32-bit", "f32", "f32"],
        vec!["64-bit", "f64", "f64"],
    ];
    print_table(headers, data);
}

fn chars() {
    log!("=========Char Type=========");
    log!("Char types are used to represent a single unicode character");
    log!("Char types are 4 bytes in size");
    log!("Char types are enclosed in single quotes");
    log!("");
    log!("         Char Type");
    print_chars();
}

fn print_chars() {
    let headers = vec!["Length", "Type"];
    let data = vec![
        vec!["4 bytes", "char"],
    ];
    print_table(headers, data);
}

fn booleans() {
    log!("=========Boolean Type=========");
    log!("Boolean types are used to represent a truth value");
    log!("Boolean types are 1 byte in size");
    log!("Boolean types are either true or false");
    log!("");
    log!("         Boolean Type");
    print_booleans();
}

fn print_booleans() {
    let headers = vec!["Length", "Type"];
    let data = vec![
        vec!["1 byte", "bool"],
    ];
    print_table(headers, data);
}

fn unit() {
    log!("=========Unit Type=========");
    log!("Unit types are used to represent the absence of a value");
    log!("Unit types are 0 bytes in size");
    log!("Unit types are written as ()");
    log!("");
    log!("         Unit Type");
}

fn main() {
    log!("");
    integer_types();
    log!("");
    floats();
    log!("");
    chars();
    log!("");
    booleans();
    log!("");
    unit();
}

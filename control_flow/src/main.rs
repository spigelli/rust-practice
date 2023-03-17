fn if_expressions() {
    if true {
        println!("This is true");
    } else if false {
        println!("This is not false");
    } else {
        println!("This is false");
    }
}

fn if_in_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn loops() {
    let mut counter = 0;
    let result = loop {
        if counter >= 10 {
            break counter;
        }
        println!("Counter is {}", counter);
        counter += 1;
    };
    println!("The result of the loop was: {}", result);
}

fn loop_labels() -> String {
    // This is super cool
    let mut grid: String = "".to_owned();
    let num_rows = 5;
    let num_cols = 5;
    let mut row = 0;
    let mut col: i32;
    'rows: loop {
        col = 0;
        loop {
            if row == num_rows {
                // In this case all rows and columns have been printed
                break 'rows;
            }
            if col == num_cols {
                // In this case all columns have been printed for the current row
                break;
            }
            grid = format!("{}{}", grid, " * ");
            col += 1;
        }
        grid = format!("{}{}", grid, "\n");
        row += 1;
    };
    grid
}

fn while_loops() {
    let a = [1, 2, 3, 4];
    let mut index = 0;
    while index < 4 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
}

fn for_in_loops() {
    let a = [1, 2, 3, 4];
    for element in a {
        println!("The value is: {}", element);
    }
}

fn ranges() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn main() {
    println!("Hello, world!");
    if_expressions();
    if_in_let();
    loops();
    println!("{}", loop_labels());
    while_loops();
    for_in_loops();
    ranges();
}



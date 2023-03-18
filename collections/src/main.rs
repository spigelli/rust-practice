fn main() {
    println!("Hello, world!");
    vectors();
    functional();
}

fn vectors() {
    let mut v:Vec<i32> = vec![1, 2, 3];
    v.push(4);
    v.push(5);

    // This would panic if the index is out of bounds
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    
    // This would return None if the index is out of bounds
    let third: Option<&i32> = v.get(2);

    // Then we could use match to handle the None case
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }
}

fn functional() {
    let sizes: String = String::from(
        "1,2;4,2;1,1"
    );
    // The dimensions list is a vector of tuples
    let dimensions: Vec<(u32, u32)> = parse_sizes(sizes);
    dimensions.iter().map(|(width, height)| {
        Rectangle {
            width: *width,
            height: *height,
        }
    }).for_each(|rect| {
        println!("The area of the rectangle is {}", rect.get_area());
    });
}

fn parse_sizes(sizes: String) -> Vec<(u32, u32)> {
    let mut dimensions: Vec<(u32, u32)> = Vec::new();
    for size in sizes.split(';') {
        let mut split = size.split(',');
        if let Some((width, height)) = split.next().zip(split.next()) {
            dimensions.push((width.parse().unwrap(), height.parse().unwrap()));
        } else {
            // Throw an error
            panic!("Invalid size: {}", size);
        }
    }
    dimensions
}
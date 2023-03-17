fn main() {
    println!("Hello, world!");
}

fn ownership_rules() {
    println!("Ownership rules");
    println!("- Every value has an *owner*");
    println!("- There can only be one owner at a time");
    println!("- When the owner goes out of scope, the value will be dropped");
}

fn variable_scope() {
    // Creating a scope
    { // s not declared
        let s = "hello"; // s is declared
        println!("{}", s);
    } // s goes out of scope and is dropped
}

fn string_type() {
    // Examples of a string literal
    let s = "hello";
    // s is immutable and a string literal
    
    // Examples of a String type
    let mut s = String::from(s);
    s.push_str(", world!");
    println!("{}", s);
}

fn data_move() {
    {
        let x = 5;
        let y = x;
    }

    {
        // ==Stack==
        // x: 5
        // y: 5
        
        let s1 = String::from("hello");
        let s2 = s1;
        
        // ==Heap==
        //         s1                
        //  __________________       
        // | NAME     | VALUE |      
        // |----------|-------|      
        // | ptr      |       |--,   
        // | len      |   5   |  |   _______________ 
        // | capacity |   5   |  |  | INDEX | VALUE |
        // '------------------'  |  |-------|-------|
        //                       |  |   0   |   h   |
        //         s2            |->|   1   |   e   |
        //  __________________   |  |   2   |   l   |
        // | NAME     | VALUE |  |  |   3   |   l   |
        // |----------|-------|  |  |   4   |   o   |
        // | ptr      |       |__|  '---------------'
        // | len      |   5   |      
        // | capacity |   5   |      
        // '------------------'      
        
        // S1 IS NO LONGER USABLE
        // S1 MOVED TO S2
    }
    
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        // ==Heap==
        //         s1                _______________
        //  __________________      | INDEX | VALUE |
        // | NAME     | VALUE |     |-------|-------|
        // |----------|-------|     |   0   |   h   |
        // | ptr      |       |---->|   1   |   e   |
        // | len      |   5   |     |   2   |   l   |
        // | capacity |   5   |     |   3   |   l   |
        // '------------------'     |   4   |   o   |
        //                          '---------------'
        //         s2                _______________ 
        //  __________________      | INDEX | VALUE |
        // | NAME     | VALUE |     |-------|-------|
        // |----------|-------|     |   0   |   h   |
        // | ptr      |       |---->|   1   |   e   |
        // | len      |   5   |     |   2   |   l   |
        // | capacity |   5   |     |   3   |   l   |
        // '------------------'     |   4   |   o   |
        //                          '---------------'
    }
}

fn consume_example() {
    fn consume_string(s: String) {
        println!("{}", s);
    }

    let s = String::from("hello");
    
    consume_string(s);
    // s is no longer usable
}

fn take_and_give_back() {
    fn take_and_give_back(s: String) -> String {
        s
    }

    let s = String::from("hello");
    let s = take_and_give_back(s);
    println!("{}", s);
}
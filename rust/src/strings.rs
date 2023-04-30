/**
 * primitive str - immutable somewhere in memory
 * String type growable heap allocated data structure - 
 *     when you need to modify or own
 */

pub fn run() {
    let mut hello = String::from("Hello ");

    // get length
    println!("Length: {}", hello.len());

    // push char
    hello.push('W');
    
    // push string
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // empty?
    println!("Is empty: {}", hello.is_empty());

    // contains
    println!("Contains 'Woerld' {}", hello.contains("World"));

    // Replace
    println!("Replace {}", hello.replace("World", "There"));

    // loop through by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
pub fn run() {
    // print to console
    println!("Hello from the print.rs file");

    // basic formatting
    println!("{} is from {}", "Robert", "WI");

    // positional arguments
    println!("{0} is from {1}, and {0} likes {2}", "Robert", "WI", "to cook");

    // named arguments
    println!("{name} likes to play {game}", name="Me-person", game="Civilization 6");

    // Placeholder traits
     println!("Binary: {:b}, hex: {:x}, octal: {:o}", 10, 10, 10);

     // Placeholder for debug
     println!("{:?}", (12, true, "Coconut laden swallow"));

     // Basic math
     println!("10 + 10 = {}", 10 + 10);
}
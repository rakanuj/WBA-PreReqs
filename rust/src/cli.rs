use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    println!("Args: {:?}", args);
    println!("Command: {}", command);

    let h: String = String::from("hello");
    match command {
        h => println!("Yay!")
    }
}
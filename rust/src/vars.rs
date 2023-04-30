// variables are primitive data or references
// variables are immutable by default
// rust is block-scoped

pub fn run() {
    let name = "Robert";
    let mut age = 43;

    println!("My name is {} and I am {}", name, age);
    
    age = 44;
    
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;

    println!("ID: {}", ID);

    // Assign multile vars
    let (my_name, my_age) = ("Robert", 11);

    println!("{} is {}", my_name, my_age);
}
// tuples group together values
// maximum 12 elements (not sure why)

pub fn run() {
    let person: (&str, &str, i8) = ("Robert", "Atlanta", 43);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}
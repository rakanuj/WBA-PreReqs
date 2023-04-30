// vectors are resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Re-assign value:
    numbers[2] = 24;

    // add to vector
    numbers.push(5);
    numbers.push(6);

    // pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // length
    println!("Vector length: {}", numbers.len());

    // arrays are stack allocated
    println!("Memory occupied {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..3];

    println!("Slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop and mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers vec: {:?}", numbers);
}
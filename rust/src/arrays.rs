// arrays are fixed length of same data type 

use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    // Re-assign value:
    numbers[2] = 24;

    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // length
    println!("array length: {}", numbers.len());

    // arrays are stack allocated
    println!("Memory occupied {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..3];

    println!("Slice: {:?}", slice);
}
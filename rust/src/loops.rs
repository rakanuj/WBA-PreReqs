/**
 * loops are used to iterate until a condition is met.
 */

pub fn run() {
    // let mut count = 0;

    //infinite loop 
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    // white loop (fizzbuzz)
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("FizzBuzz");
    //     } else if count % 5 == 0 {
    //         println!("Buzz");
    //     } else if count % 3 == 0 {
    //         println!("Fizz");
    //     } else {
    //         println!("{}", count);
    //     }

    //     // inc
    //     count += 1;
    // }

    // for range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", x);
        }
    }
}
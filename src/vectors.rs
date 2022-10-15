// Vectors - Resisable Arrays

use core::num;
use std::mem;

pub fn run() {
    println!("### Vectors ###");

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(6);
    numbers.push(7);

    // Pop off last value
    numbers.pop();


    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Vector Length: {}", numbers.len());

    // Arrays are stack allocated. &numbers passes a reference to the array
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values 
    for x in numbers.iter_mut() {
        // Multiply by 2
        *x *= 2;
    }

    println!("Numbers Vector: {:?}", numbers);
}
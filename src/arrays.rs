// Arrays - Fixed list where elements are the same data types.
/*
Documentation: https://doc.rust-lang.org/book/ch03-02-data-types.html
- Fixed length.
- Useful when you want data to be allocated on a stack rather than heap. 
- Useful when you want to ensure that you have fixed number of elements. 
- The number of elements will not change. 

Runtime error possible with arrays: 'index out of bounds'. Program still compiles.
- To avoid it can implement checks. 
*/
use std::mem;

pub fn run() {
    println!("### Arrays ###");

    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated. &numbers passes a reference to the array
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Initalize array with the same value for each element. 
    // Creates array: [3, 3, 3, 3, 3]
    let a = [3; 5];
    println!("Array initalized with the same element: {:?}", a);
}
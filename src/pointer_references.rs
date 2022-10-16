// Reference Pointers - point to the resource in memory
pub fn run() {
    println!("### Reference Pointers ###");

    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Array values: {:?}", (arr1, arr2));


    /* With non-primitives, if you assign another variable to a
    piece of data, the first variable will no longer hold that value.
    You will need to use a reference (&) to point to the resource.
    Vectors don't implement the copy trait. 
     */
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Vector Values: {:?}", (&vec1, vec2));


}
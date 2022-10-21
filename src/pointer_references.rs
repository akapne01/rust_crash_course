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

    // Call function and transfer ownership of the String to it
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    // References
    let s1 = String::from("hello");
    // &s1 allows us to create a reference that refers to s1 value, 
    // but does not own it. Because it does not own it, the value it 
    // points to will not be dropped when the reference stops being used.
    let len = calculate_length_pass_reference(&s1);
    // Passing reference to s1 makes it still in scope here, because it 
    // doesn't transfer ownership of the String, so it doesn't go out of scope.
    println!("The length of '{}' is {}.", s1, len);
}

// &String allows you to refer to some value without taking ownership of it.
// The opposite of referencing is dereferencing, which is accomplished with 
// dereference operator:`*`. 
fn calculate_length_pass_reference(s: &String) -> usize {
    // When functions have references as parameters instead of actual values,
    // we won't need to return the values in order to give back ownership,
    // because we never had the ovnership.
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

/// Reference: https://doc.rust-lang.org/rust-by-example/fn/closures.html
/// Closures are functions that can capture the enclosing environment.
/// Clousure that captures the variable x is: |val| val + x
/// Calling a closure is exactly like calling a function.
/// Both input and return types can be inferred and input variable names must be specified.
/// Clousures use |x| around the variables.
/// Optional body delimination '{}' for a single expression, mandatory otherwise.
/// The ability to capture the outer envvars => Can use all variables that are in the scope.
pub fn run() {
    println!("### Closures ###");

    // Increment using a function
    fn increment(i: i32) -> i32 {
        i + 1
    }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional as
    // as the '{}' wrapping the body.
    // These nameless functions are assigned to appropriately names variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    println!("Function: {}", increment(i));
    println!("closure_annotated: {}", closure_annotated(i));
    // Once clousure is inferred, it cannot be inferred again with another type.

    println!("closure_inferred: {}", closure_inferred(i));

    // A clousure taking no args, returns an 'i32'
    let one = || 1;
    println!("clousure returning one: {}", one());
}

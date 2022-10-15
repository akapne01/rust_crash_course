// Used to store blcks of code for re-use. 

pub fn run() {
    println!("### Functions ###");
    greeting("Hello", "Jane");
    
    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Clousure -> with closures can use outside variables which we can't do with f(x)
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name:&str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    // If we don't use semicolon, that is telling what we want returned.
    n1 + n2
}
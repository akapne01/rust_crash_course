use core::num;

/// References:
/// https://www.newline.co/@kkostov/the-rust-map-function-a-gateway-to-iterators--f991b22b
/// https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter
///
/// Map function is core of functional programming.
/// Alllows to use iterator to modify each element in a list.
///
/// # iter() # iterates over titems by reference.
/// The iterator will yield &T by convention.
/// Ad-hoc method. Return type is independent from the context.
///
/// # into_iter() # iterates over items, mocing them into the new scope.
/// May yield any of: T, &T, or &mut T.
/// Comes from IntoIterator trait. Implemented when you want to specify how a particular
/// type is to be converted into an iterator.
/// Can be used in for loop.
/// Generic method to obtain iterator. Wheater it yield values, immutable refs or
/// mutable refs is context dependent and can sometimes be suprising.
///
/// # iter_mut() # iterates over the items, giving a mutable reference to each of them.
/// Yields &mut T by convention.
/// Ad-hoc method. Return type is independent from the context.
///
/// When method is not implemented for a value, it is automatically searched for references to that value instead.
///
/// ## Map ##:
/// - Can only be applied to iterators. First need to convert collection to iterrator.
/// - Is lazy. The execution of the closure provided to map is delayed until the values
/// produced by the map are actually requested.

pub fn run() {
    println!("### Map function in Rust");
    let numbers = vec![3, 6, 9, 12];
    let mut number_of_times = 0;
    // Multiply each number by 10
    // Map method executes a closure over each element yielded by iterator.
    let result = numbers.iter().map(|n| {
        number_of_times += 1;
        return n * 10;
    });

    // map() returns an iterrator. Result of the first map transformation will be used
    // as input for the 2nd map transformation.
    // collect() method acts as a consumer to the iterator, acquiring all elements and
    // storing them into vector.
    let result2: Vec<i32> = numbers.iter().map(|n| n * 10).map(|n| n / 3).collect();
    /*
    We have 2 iterators at play.
    1) iter()
    2) Which happens to be the return type of the `.map()` function. Map returns an itterator.
    */
    println!("Res: {:?}", result);
    /*
    Res: Map { iter: Iter([3, 6, 9, 12]) }
    Number of times: 0
    It's not executed yet. Lazy nature of map() allows to control when the transformation will
    be executed.
    */
    println!("Number of times: {}", number_of_times);
    println!("Res: {:?}", result2);

    // Tranform vector of Strings to lowercase.
    let words = vec!["Hello", "from", "Rust", "!"];
    println!("Words before map: {:?}", words);
    let lowercased_words: Vec<String> = words.iter().map(|word| word.to_lowercase()).collect();
    println!("Words after map: {:?}", lowercased_words);

    // Count the number of times a character occurs in a String
    let text = String::from("hello rom rust!");
    let result = "abcdefghijklmnoprstuvwxyz"
        .chars()
        .map(|c| (c, text.matches(c).count()))
        .collect::<std::collections::HashMap<_, _>>();
    println!("{:?}", result);

    // Map usign Option or Result, when mapping may not suceed.
    // Option is an enum type with 2 possible values: None or Some(value). Can be used to
    // indicate that a given operation produces a value or not.
    // Result is an enum type with 2 values: Or or Err which can be used to indicate if a given
    // operation has succeeded of failed in lieu of throwing an exception (or a panic).
    // `unwrap()` can be used, but it will only work if no errors expected.
    // .map(|str_number| str_number.parse::<u32>().unwrap()) if not panicking, the program will work.
    // If there are any values that may not compult then can use `flat_map()`. It can handle the situation
    // where the Map fucntion can produce one or zero results.
    let str_numbers = vec!["1", "2", "3", "I am not a number"];
    let numbers: Vec<u32> = str_numbers
        .iter()
        .flat_map(|str_number| str_number.parse::<u32>())
        .collect();
    println!("numbers: {:?}", numbers); // numbers: [1, 2, 3] Errors are disregarded.

    // # flat_map() #
    // - Uses an iterator over the result of the mapping and as a consequence, it will skip over elements for which
    // the mapping closure returns empty or unsuccessful values (like None or Err).
    // - Allows to skip over the failing elements and end up with resulting vector of numebrs for which the operation suceeded.

    /*
    # Side effects using map() #
    - During the execution of a Map closure or a fucntion, our program may modify state and variables which are external. Like
    in a case where we where incrementing counter, every time a map function was executed.
    - In a case of counter it was expected, but it may not always be a case.
    */

    // Let's modify example so that instead of transforming the numebr in the vector, we map the order in which they are processed.
    let nums = vec![3, 6, 9, 12];
    let mut num_of_times = 0;
    let res: Vec<(i32, i32)> = nums
        .into_iter()
        .map(|n| {
            num_of_times += 1;
            return (n, num_of_times);
        })
        .rev() // Reverses the iterator returned by map
        .collect();
    println!("Res: {:?}", res); // Res: [(12, 1), (9, 2), (6, 3), (3, 4)]

    // To have better control of operations with side effects, it may be freferred to use the loop instead:
    let nums = vec![3, 6, 9, 12];
    let mut num_of_times = 0;
    let mut result = Vec::new();
    for n in nums.into_iter() {
        num_of_times += 1;
        result.push((n, num_of_times));
    }
    println!("Res: {:?}", result);
}

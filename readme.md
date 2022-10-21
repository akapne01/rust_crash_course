# Crush course in Rust
Created following the Youtube video: https://www.youtube.com/watch?v=zF34dRivLOw
This youtube video is a layer 1, a quick introduction.

# Rust official documentation
https://doc.rust-lang.org/book/
The knowledge is then deepened by reading an offical Rust documentation.

# Understanding Ovenrship in Rust
- Reference: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html 
- Enables Rust to make memory safety guarantees without needing a garbage collector.
- Ownership is a set off rules that governs how Rust manages memory. 
- There are at least 3 approaches how programms memory can be managed at runtime.
    1) Programmer explicitly allocates and frees the memory as in C. 
    2) Garbege Collector can take care of the memory allocation, as in Java, Python, Scala. 
    3) Memory is managed through a system of ownership with a set of rules that the compiler checks. If any rule violated, the prgram doesn't compile.

# The Stack and Heap
- Weather value is on the stack or the heap affects how program behaves.
- Both Stack and Heap are parts of memory available to your code to use at runtime, butt they are structured in different ways.
- Generally stack is faster then heap to allocate and access information. 
- When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function's local variables get pushed onto the stack. When the function is over, those values get popped off the stack. 
- Ownership addresses the following pronlems::
        * Keeping track of what parts of code are using what data on the heap, 
        * Minimizing the amount of duplicate data on the heap, 
        * Cleaning up unused data on the heap so that you don't run out of space
- Once you understand ownership, you don't need to think about the stack and heap very often.
- Knowing that the main purpose of ownership is to manage heap data can help to explain why it works the way it does.

# Stack
- Stack stores values in a manner: last in, first out. Like stack of plates.
- Pushing into the stack means adding value form the stack. 
- Popping of the stack means removing value from the stack. 
- All data stored in stack must have a known, fixed size. 
- Data with an unknown size at compile time or size that might change, must be stored on heap instead.
- Pusing to the Stack is faster than allocating on the Heap, becasue the allocator never has to search for a place to store the new data; that location is always at the top of the stack.
- Variabls stored on the stack can be quickly and trivially copied and to make a new, independent instance if another part of code needs to use the same value in a different scope.

# Heap
- Is less organized than stack. 
- When you put data on the heap, you request a certain amount of space.
- The momory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.
- This process is called allocating on the heap and is soemtimes abreviated as just allocating (pushing values onto the stack is not considered allocating).
- Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack.
- When you want the actual data, you must follow the pointer. 
- Think of this as being seated in the restaurant. When you enter you state the number of people in your group, and the staff finds an empty table that fits the number of people and leads you there. If someone in your group is late, they can ask where you have been seated to find you.
- The heap requires more work, because the allocator must find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
- Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. 

# Owneership Rules
- Each value is Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

# Variable Scope
- A scope is the range within a program for which an item is valid.
- A variable is valid when it comes into scope and remains valid until it goes out of scope.
- Is defined by {}

# String data type stores data on heap
- str is a primitive data type, is stored on the stack.
- We want to explore how data is allocated on the heap and as such is able to store an amount of text that is unknown to us at the compile time.
- To create a string from a literal: `let s = String::from("hello");`
- To make String mutable, add a keyword `mut`: `let mut s = String::from("hello");`.
- The differece between `str` and `String` is how they both deal with memory. 
- `str` the contents are known at the compile time, so the text is hardcoded directly into the final executable. This makes `str` lierals fast and efficient, because it is immutable. 
- With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means that:
    1) Memory must be requested from the memory allocator at runtime. 
    2) We neeed a way of returning  this memory to allocator when we are done with our `String`. 
- When we call `String::from`, its implementation requests the memory it needs. This part is pretty much universal in programming languages. The second part is different.
- In languags with a garbedge collector (GC), the GC keeps track of and cleans up memory that is not used anywhere, and we don't need to think about it. 
- In most languages without the GCm it's the programmers responsibility to identify when memory is no longer being used an call code to explicitly free it, just as we did to request it. Doing this correctly historically has been a difficult programming problem. If we forget, we will waste memory. If we do it too early, we have an invalid variable. If we do it twice, it's a bug too. We need to pair exactly one `allocate` with exactly one `free`. 
- Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. There is a natural point at which we can return the memory of our `String` needs to the allocator: when `s` goes out of the scope.:

<code>
            {
                    let s = String::from("hello"); // s is valid from this point forward

                    // do stuff with s
                }                                  // this scope is now over, and s is no
                                                // longer valid
</code>

- When a variable goes out of scope, Rust calls a special fucntion for us called `drop`, and it's where the author of `String` can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.
- It uses Reasource Acquisition Is Initialization (RAII) pattern. This pattern has profound impact on the way Rust code is written.

# Ways Varaibles and Data Interact: Move
Multiple variables can interact with the same data in different ways in Rust. 

            let x = 5;
            let y = x;

        - Binds the value `5` to `x`
        - Then make a copy of the value `x` and bind it to `y`.
        - Becuase they are int32, they live on the stack. 


            let s1 = String::from("hello");
            let s2 = s1;
        
        - A String is made of 3 parts:
            1) A pointer to the memory that holds the contents if the string.
            2) Length of the String `len`
            3) Capacity
        - This group of data is stored on the stack. 
        - Memory on the heap hold the actual contents of the String. String is stored as an index value pair, or an array.
        - `len` is how much memory, in bytes, the contents of the String is currently using.
        - The `capacity` is the total amount of memory, in bytes, that the String has recieved from allocator. 
        - When we assign `s1` to `s2`, the `String` data is copied, meaning we copy the pointer, the length, and capacity that are on the stack. We do not copy the data on the heap that the pointer refers to. `s1` and `s2` point to the same location in heap memory.
        - Because 2 variables are pointing to the same data when s1 goes out of scope, so does s2. They will both try to free the same memory and that is known as a double free error. Freeing memory twice can lead to data corruption, which can potentially lead to security voulnerabilities. To ensure memory safety, after the line `let s2 = s1`, Rust considers `s1` as no longer valid. 
        - If you have heared about terms shallow and deep copy in other programing languages, the concept of copying the pointer, length, capacity without copying the data sounds like making a shallow copy. But because Rust invalidates the first variable, instead of calling it a shallow copy, it's know as a move. In this example, we would say that s1 was moved into s2. Now only s2 is valid, when it goes out of scope, it alone will free the memory.
        - There's a design choice that's implied by this: Rust will never automatically create "deep" copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime preformance. 

# Ways Variables and Data Interact: Clone
- If we do want to deeply copy the heap data of the String, not just the stack data, we use a common method called: `clone`.
- When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive. It's a visual indication that soemthing different is going on. 

# Stack-Only Data: Copy
- Primitives, such as intergers known at compile time are stored entierly on the stack. Copies of actual values are quick to make.
- In other words, there is no difference between deep and shallow copy with primitives. 
- Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the stack, as integers are.
- If a type implements `Copy` trait, varaibles that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable. 
- Rust will not let us annotate a type with `Copy` if the type, or any of its parts, has implemented a `Drop` trait. If a type needs soemthing special to happen when the value goes out of scope and we add the `Copy` annotation to that type, we will get a compile-time error. To learn about how to add the `Copy` annotation to your type to implement the trait, see Derivable Traits, in Appendix C in the book. 
- What types implement the `Copy` trait? 
        1) All integer types
        2) Boolean type
        3) All floating point types
        4) The character type
        5) Tuples, but only if contain types that also implement `Copy`. For example `(i32, i32)` implements `Copy`, but `(i32, String)` does not.

# Ownership and Functions
- The mechanics of passing a value to a function are similar to those when assigning a value to a variable.
- Passing a variable to a function will move or copy, just as assignment does. 

Example:
                fn main() {
                    let s = String::from("hello");  // s comes into scope

                    takes_ownership(s);             // s's value moves into the function...
                                                    // ... and so is no longer valid here

                    let x = 5;                      // x comes into scope

                    makes_copy(x);                  // x would move into the function,
                                                    // but i32 is Copy, so it's okay to still
                                                    // use x afterward

                } // Here, x goes out of scope, then s. But because s's value was moved, nothing
                // special happens.

                fn takes_ownership(some_string: String) { // some_string comes into scope
                    println!("{}", some_string);
                } // Here, some_string goes out of scope and `drop` is called. The backing
                // memory is freed.

                fn makes_copy(some_integer: i32) { // some_integer comes into scope
                    println!("{}", some_integer);
                } // Here, some_integer goes out of scope. Nothing special happens.

End of Example. 

- If tried using `s` after the call to `takes_ownership`, Rust would throw a compile-time error. These static checks protect us from mistakes. 

# Return Values and Scope
- Returning values can also transfer ownership. 
Example:
            fn main() {
                let s1 = gives_ownership();         // gives_ownership moves its return
                                                    // value into s1

                let s2 = String::from("hello");     // s2 comes into scope

                let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                                    // takes_and_gives_back, which also
                                                    // moves its return value into s3
            } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
            // happens. s1 goes out of scope and is dropped.

            fn gives_ownership() -> String {             // gives_ownership will move its
                                                        // return value into the function
                                                        // that calls it

                let some_string = String::from("yours"); // some_string comes into scope

                some_string                              // some_string is returned and
                                                        // moves out to the calling
                                                        // function
            }

            // This function takes a String and returns one
            fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                                // scope

                a_string  // a_string is returned and moves out to the calling function
            }
- The ownership of a variable follows the same pattern every single time: assigning a value to another variable moves it. 
- When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable. 
- Anything we pass in to the function also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well. Rust allows to return multiple values using tuples.
    Example: 
            fn main() {
                let s1 = String::from("hello");

                let (s2, len) = calculate_length(s1);

                println!("The length of '{}' is {}.", s2, len);
            }

            fn calculate_length(s: String) -> (String, usize) {
                let length = s.len(); // len() returns the length of a String

                (s, length)
            }

- Rust has a feature for using a value without transferring ownership, called references.

# References and Borrowing
- The issue with the tuple code above is that we have to return the String to the calling function so we can still use the String after the call to calculate_length, because the String was moved into calculate_length. 
- Instead, we can provide a reference to the String value.
- A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
Example: Here is how you would define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership of the value:

            fn main() {
                let s1 = String::from("hello");
                // passing a reference to s1 to the function
                let len = calculate_length(&s1);
                // Passing a reference instead of String itself makes both s1, and len in scope here.  
                println!("The length of '{}' is {}.", s1, len);
            }

            // Function expects a reference to the String type and returns the length of the String. 
            fn calculate_length(s: &String) -> usize {
                s.len()
            }

- We call the action of creating a reference borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.
- You can't modify soemthing you have borrowed. Just as variables are immutable by default, so are references. We are not allowed to modify somthing we have a reference to. 

# Mutable References
- To create a mutable reference, the variable needs to be defined as mutable. Then we pass `&mut` to the function call. The function call expects `&mut String` variable. This makes it clear that the function will mutate the reference it borrows.
- Mutable references have one restriction: if you have a mutable reference to a value, you can have no other references to that value. This restriction allows for mutation, but in more controlled fashion, without unexpected results. 
- Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these 3 behaviours occur:
        1) Two or more pointers access the same data at the same time.
        2) At least one of the pointers is being used to write to the data.
        3) There is no mechanism being used to syncronize access to the data.
        Data Races cause undefined behaviour and can be difficult to diagnoze and fix when you are trying to track them down at runtime. 
- Rust prevents data race problem by refusing to compile code with data races. 
- We also cannot have a mutable reference while we have an immutable one to the same value. 
- Multiple immutable references are allowed, becasue no one who is just reading the data has the ability to affect anyone else's reading of the data. 

- Note that reference scope starts from where it is introduced and continues through the kast time that reference is being used.
- The ability of the compiler to tell that a reference is no longer being used at a point before the end of the scope is called Non-Lexical Lifetimes (NNL for short). You can read more about it here: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#non-lexical-lifetimes

# Dangling References
- In languages with pointers it is easy to make a dangling pointer by freeing memory, but preserving a pointer to that memory. 
- A dangling pointer is a pointer that refrences a location in momory that may have been given to someone else.
- In Rust the compiler guarantees that references will never be dangling; if you have a reference to some data, the compiler will ensure that data will not go out of scope before the reference to the data does.
- Error shown by Rust compiler: this function's return type contains a borrowed value, but there is no value for it to be borrowed from

# The Rules of References
Let’s recap what we’ve discussed about references:
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

# The Slice Type
- Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.


// Lesson 1: Rust Basics - Hello World and Variables
// To compile and run: rustc lesson1.rs && ./lesson1 (or lesson1.exe on Windows)

fn main() {
    // This is the main function - every Rust program starts here
    println!("Hello, Rust world!");
    
    // VARIABLES AND MUTABILITY
    // By default, variables in Rust are immutable (cannot be changed)
    let x = 5;
    println!("The value of x is: {}", x);
    
    // This would cause a compile error:
    // x = 6; // Cannot assign twice to immutable variable
    
    // To make a variable mutable, use the 'mut' keyword
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 15; // This is allowed because y is mutable
    println!("The new value of y is: {}", y);
    
    // VARIABLE SHADOWING
    // You can declare a new variable with the same name as a previous variable
    let z = 20;        // Create first variable 'z' with value 20
    let z = z + 5;     // Create NEW variable 'z' with value (old z + 5) = 25
    let z = z * 2;     // Create ANOTHER NEW variable 'z' with value (previous z * 2) = 50
    println!("The value of z is: {}", z); // Should print 50

    // Type changes
    // let spaces = "   ";        // string
    // let spaces = spaces.len(); // number (same name, different type)

    // Transformation pipelines
    // let input = "123";
    // let input = input.trim();      // remove whitespace
    // let input = input.parse::<i32>().unwrap(); // convert to number
    // unwrap() extracts a value from a Result or Option type, but crashes the program if there's an error.
    // Many operations in Rust can fail, so they return special types:
    // Result<T, E> - for operations that might succeed (Ok) or fail (Err)
    // Option<T> - for values that might exist (Some) or not exist (None)
    // When to use unwrap():
    //     Learning/prototyping: When you're just experimenting
    //     When you're 100% sure it won't fail: Very rare in real code
    //     Examples and tutorials: To keep code simple


    // Stack: Fast, limited size, automatic cleanup, stores fixed-size data
    // Heap: Slower, large size, manual management, stores variable-size data
    
    // CONSTANTS
    // Constants are always immutable and must have a type annotation
    const MAX_POINTS: u32 = 100_000;
    println!("Maximum points: {}", MAX_POINTS);
    
    // DATA TYPES
    // Rust is statically typed, but can often infer types
    
    // Integers
    let small_number: i32 = -42;           // 32-bit signed integer
    let big_number: u64 = 42;              // 64-bit unsigned integer
    let default_int = 42;                  // defaults to i32
    
    println!("Small number: {}", small_number);
    println!("Big number: {}", big_number);
    println!("Default int: {}", default_int);
    
    // Floating point
    let pi: f64 = 3.14159;                 // 64-bit floating point
    let also_pi: f32 = 3.14159;            // 32-bit floating point
    let default_float = 2.71828;           // defaults to f64
    
    println!("Pi (f64): {}", pi);
    println!("Pi (f32): {}", also_pi);
    println!("Default float: {}", default_float);
    
    // Boolean
    let is_rust_awesome: bool = true;
    let is_learning_fun = false;           // type inferred
    
    println!("Is Rust awesome? {}", is_rust_awesome);
    println!("Is learning fun? {}", is_learning_fun);
    
    // Character (note: single quotes for char, double quotes for string)
    let letter: char = 'R';
    let emoji: char = '🦀'; // Rust's mascot is a crab!
    
    println!("Letter: {}", letter);
    println!("Emoji: {}", emoji);
    
    // STRING TYPES
    // String literals (stored in the program binary)
    // "Stored in program binary" means the string is embedded directly in your executable file and loaded into a read-only section of memory when your program starts.
    let greeting = "Hello, Rust!";
    println!("Greeting: {}", greeting);
    
    // String type (growable, heap-allocated)
    let mut dynamic_string = String::new();
    dynamic_string.push_str("This string can grow!");
    println!("Dynamic string: {}", dynamic_string);
    
    // BASIC ARITHMETIC
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);
    
    // EXERCISES FOR YOU TO TRY:
    // 1. Create a mutable variable called 'counter' and increment it several times
    // 2. Create constants for your favorite number and print it
    // 3. Try different number types (i8, i16, u8, u16, etc.)
    // 4. Practice with string operations
    
    println!("\n--- End of Lesson 1 ---");
    println!("Next lesson will cover: Functions, Control Flow, and Collections");
}

// LESSON 1 SUMMARY:
// - Every Rust program starts with a main() function
// - Variables are immutable by default, use 'mut' to make them mutable
// - Variable shadowing allows reusing variable names
// - Constants are always immutable and need type annotations
// - Rust has various data types: integers, floats, booleans, characters, strings
// - println! macro is used for printing (note the exclamation mark!)
// - Rust is statically typed but can infer types in many cases

// TO RUN THIS LESSON:
// 1. Save this file as lesson_1.rs
// 2. Open terminal/command prompt
// 3. Navigate to the directory containing the file
// 4. Compile: rustc lesson_1.rs
// 5. Run: ./lesson_1 (Linux/Mac) or lesson_1.exe (Windows)
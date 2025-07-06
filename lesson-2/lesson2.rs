// Lesson 2: Functions, Control Flow, and Collections
// To compile and run: rustc lesson2.rs && ./lesson2 (or lesson2.exe on Windows)

fn main() {
    println!("=== Lesson 2: Functions, Control Flow, and Collections ===");
    
    // FUNCTIONS
    println!("\n--- Functions ---");
    
    // Call our custom functions
    greet_user();
    let sum = add_numbers(5, 3);
    println!("5 + 3 = {}", sum);
    
    let (quotient, remainder) = divide_with_remainder(17, 5);
    println!("17 ÷ 5 = {} remainder {}", quotient, remainder);
    
    // CONTROL FLOW - IF/ELSE
    println!("\n--- Control Flow: If/Else ---");
    
    let temperature = 25;
    
    if temperature > 30 {
        println!("It's hot outside!");
    } else if temperature > 20 {
        println!("Nice weather!");
    } else {
        println!("It's cold outside!");
    }
    
    // If expressions (if can return values)
    let weather_description = if temperature > 25 {
        "warm"
    } else {
        "cool"
    };
    println!("Today is {}", weather_description);
    
    // LOOPS
    println!("\n--- Loops ---");
    
    // Loop with break
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            break;
        }
        println!("Loop iteration: {}", counter);
    }
    
    // While loop
    let mut countdown = 3;
    while countdown > 0 {
        println!("Countdown: {}", countdown);
        countdown -= 1;
    }
    println!("Blast off! 🚀");
    
    // For loop with range
    println!("\nCounting from 1 to 5:");
    for i in 1..=5 {  // ..= includes the end number
        println!("Number: {}", i);
    }
    // 1..5 (with just ..) would be exclusive, giving you 1, 2, 3, 4 (stops before 5)
    // iterate backwards with (1..=5).rev()

    
    // COLLECTIONS
    println!("\n--- Collections ---");
    
    // ARRAYS - Fixed size, same type
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);  // {:?} is debug formatting
    println!("First number: {}", numbers[0]);
    println!("Array length: {}", numbers.len());
// {} uses the Display trait (for user-facing output)
// {:?} uses the Debug trait (for developer/debugging output)
// Many types like arrays, vectors, structs don't implement Display by default, But most types automatically implement Debug or you can derive it
// let nested = vec![vec![1, 2], vec![3, 4]];
// println!("Compact: {:?}", nested);    // [[1, 2], [3, 4]]
// println!("Pretty:\n{:#?}", nested);   // Nicely formatted across lines

    
    // Create array with same value
    let zeros = [0; 3];  // Creates [0, 0, 0]
    println!("Zeros array: {:?}", zeros);
    
    // Iterate over array
    println!("Array elements:");
    for (index, value) in numbers.iter().enumerate() {
        println!("Index {}: {}", index, value);
    }
    
    // VECTORS - Dynamic size, same type
    let mut fruits = Vec::new();
    fruits.push("apple");
    fruits.push("banana");
    fruits.push("orange");
    println!("Fruits vector: {:?}", fruits);
    
    // Alternative vector creation
    let mut scores = vec![10, 20, 30];
    scores.push(40);
    println!("Scores: {:?}", scores);
    
    // Access vector elements safely
    match scores.get(2) {
        Some(score) => println!("Third score: {}", score),
        None => println!("No third score found"),
    }
    
    // Iterate over vector
    println!("All scores:");
    for score in &scores {
        println!("Score: {}", score);
    }
    
    // STRINGS AND STRING SLICES
    println!("\n--- Strings ---");
    
    // String slice (&str) - immutable reference to string data
    let message = "Hello, Rust!";
    println!("Message: {}", message);
    
    // String - owned, mutable string
    let mut greeting = String::from("Hello");
    greeting.push_str(", World!");
    greeting.push('!');
    println!("Greeting: {}", greeting);
    
    // String methods
    println!("Greeting length: {}", greeting.len());
    println!("Greeting in uppercase: {}", greeting.to_uppercase());
    println!("Contains 'World': {}", greeting.contains("World"));
    
    // PATTERN MATCHING WITH MATCH
    println!("\n--- Pattern Matching ---");
    
    let number = 7;
    match number {
        1 => println!("One"),
        2 | 3 => println!("Two or Three"),
        4..=6 => println!("Four to Six"),
        7 => println!("Lucky Seven!"),
        _ => println!("Something else"),
    }
    
    // Match with Option
    let maybe_number = Some(42);
    match maybe_number {
        Some(value) => println!("Found a number: {}", value),
        None => println!("No number found"),
    }
    
    // PRACTICAL EXAMPLE: Grade Calculator
    println!("\n--- Practical Example: Grade Calculator ---");
    
    let grades = vec![85, 92, 78, 96, 88];
    let average = calculate_average(&grades);
    let letter_grade = get_letter_grade(average);
    
    println!("Grades: {:?}", grades);
    println!("Average: {:.1}", average);
    println!("Letter grade: {}", letter_grade);
    
    // EXERCISES FOR YOU TO TRY:
    println!("\n--- Exercises ---");
    println!("1. Write a function that takes a number and returns whether it's even or odd");
    println!("2. Create a vector of your favorite colors and print them with their positions");
    println!("3. Write a function that finds the maximum number in a vector");
    println!("4. Create a simple guessing game using loops and conditionals");
    println!("5. Practice with string manipulation: reverse a string, count vowels, etc.");
    
    println!("\n--- End of Lesson 2 ---");
    println!("Next lesson will cover: Ownership, Borrowing, and Lifetimes");
}

// FUNCTION DEFINITIONS
// Functions are defined with 'fn' keyword
fn greet_user() {
    println!("Welcome to Rust programming!");
}

// Function with parameters and return type
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b  // Note: no semicolon = return value
}

// Function returning multiple values (tuple)
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)  // Return tuple
}

// Function working with vectors (borrowing)
fn calculate_average(numbers: &Vec<i32>) -> f64 {
    let sum: i32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}

// Function with match expression
// all match statements must be exhaustive
fn get_letter_grade(average: f64) -> char {
    match average {
        90.0..=100.0 => 'A',
        80.0..=89.9 => 'B',
        70.0..=79.9 => 'C',
        60.0..=69.9 => 'D',
        _ => 'F',
    }
}

// match average {
//     90.0..=100.0 => 'A',
//     80.0..=89.9 => 'B',
//     // ... other cases
//     other => {  // Named instead of _
//         println!("Grade for {}: F", other);
//         'F'
//     }

// LESSON 2 SUMMARY:
// - Functions are defined with 'fn' and can take parameters and return values
// - Control flow: if/else expressions, loops (loop, while, for)
// - Collections: Arrays (fixed size), Vectors (dynamic size)
// - Strings: &str (string slices) vs String (owned strings)
// - Pattern matching with 'match' is powerful for handling different cases
// - Use & to borrow data instead of taking ownership
// - Ranges: .. (exclusive end), ..= (inclusive end)
// - {:?} for debug printing, {:.1} for formatted floating point
// - Functions without semicolon on last line return that value



// &str (String Slice):

// A reference to a string stored somewhere else
// Immutable by default
// Doesn't own the data - it's "borrowed"
// Fixed size (can't grow or shrink)
// Very lightweight (just a pointer + length)

// String (Owned String):

// Owns the string data
// Mutable (can be modified)
// Stored on the heap
// Can grow and shrink dynamically
// Heavier weight (manages memory allocation)
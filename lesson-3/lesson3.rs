// Lesson 3: Ownership, Borrowing, and Lifetimes

fn main() {
    println!("=== Lesson 3: Ownership, Borrowing, and Lifetimes ===");
    
    // OWNERSHIP - Rust's Most Important Feature
    println!("\n--- Ownership Rules ---");
    println!("1. Each value in Rust has a single owner");
    println!("2. There can only be one owner at a time");
    println!("3. When the owner goes out of scope, the value is dropped");
    
    // OWNERSHIP IN ACTION
    println!("\n--- Ownership Examples ---");
    
    // Simple ownership transfer (move)
    let s1 = String::from("Hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    // println!("{}", s1); // This would cause a compile error!
    println!("s2: {}", s2);
    
    // Copy vs Move
    let x = 5;
    let y = x; // Integers implement Copy trait, so both x and y are validasdf
    println!("x: {}, y: {}", x, y);
    
    // Function ownership
    let message = String::from("Hello, Functions!");
    take_ownership(message);
    // println!("{}", message); // This would error - message was moved
    
    let number = 42;
    make_copy(number); // number is copied, still valid
    println!("Number is still valid: {}", number);
    
    // BORROWING - Using References
    println!("\n--- Borrowing with References ---");
    
    let original = String::from("I'm the original!");
    let length = calculate_length(&original); // Borrow with &
    println!("'{}' has {} characters", original, length);
    
    // Mutable references
    let mut text = String::from("Hello");
    change_string(&mut text);
    println!("After modification: {}", text);
    
    // BORROWING RULES
    println!("\n--- Borrowing Rules ---");
    println!("1. You can have many immutable references OR one mutable reference");
    println!("2. References must always be valid (no dangling references)");
    
    // Multiple immutable references - OK
    let data = String::from("Shareable data");
    let ref1 = &data;
    let ref2 = &data;
    let ref3 = &data;
    println!("ref1: {}, ref2: {}, ref3: {}", ref1, ref2, ref3);
    
    // Mutable reference example
    let mut counter = 0;
    {
        let counter_ref = &mut counter;
        *counter_ref += 10; // Dereference with *
    } // mutable reference goes out of scope here
    println!("Counter after increment: {}", counter);
    
    // REFERENCE SCOPE AND LIFETIME
    println!("\n--- Reference Lifetimes ---");
    
    let result;
    {
        let num1 = 10;
        let num2 = 20;
        result = larger_number(num1, num2); // Returns a copy, not a reference
    } // num1 and num2 go out of scope, but result is fine
    println!("Larger number: {}", result);
    
    // STRING SLICES - A Special Kind of Reference
    println!("\n--- String Slices ---");
    
    let sentence = String::from("Hello wonderful world");
    let hello = &sentence[0..5];   // Slice from index 0 to 5 (exclusive)
    let world = &sentence[16..21]; // Slice from index 16 to 21 (exclusive)
    println!("First word: {}", hello);
    println!("Last word: {}", world);
    
    // Finding first word with slices
    let first_word = get_first_word(&sentence);
    println!("First word function result: {}", first_word);
    
    // Array slices
    let numbers = [1, 2, 3, 4, 5];
    let middle = &numbers[1..4]; // [2, 3, 4]
    println!("Middle slice: {:?}", middle);
    
    // PRACTICAL OWNERSHIP PATTERNS
    println!("\n--- Practical Ownership Patterns ---");
    
    // Pattern 1: Return ownership to keep using values
    let mut words = vec!["apple", "banana", "cherry"];
    words = add_word_and_return(words, "date");
    println!("Words after adding: {:?}", words);
    
    // Pattern 2: Use references to avoid moving
    let fruits = vec!["apple", "banana", "cherry"];
    let count = count_items(&fruits);
    println!("Fruits: {:?}, Count: {}", fruits, count);
    
    // Pattern 3: Clone when you need independent copies
    let original_list = vec!["red", "green", "blue"];
    let copied_list = original_list.clone();
    println!("Original: {:?}", original_list);
    println!("Copy: {:?}", copied_list);
    
    // COMMON OWNERSHIP MISTAKES AND SOLUTIONS
    println!("\n--- Common Mistakes and Solutions ---");
    
    // Problem: Trying to use value after move
    let data1 = String::from("Important data");
    // Solution: Use references or clone
    process_data(&data1); // Borrow instead of move
    println!("Data1 is still accessible: {}", data1);
    
    // Problem: Multiple mutable references
    let mut score = 100;
    // Solution: Use references in different scopes
    {
        let score_ref = &mut score;
        *score_ref -= 10;
    } // First mutable reference ends here
    {
        let another_ref = &mut score;
        *another_ref += 5;
    } // Second mutable reference ends here
    println!("Final score: {}", score);
    
    // WORKING WITH VECTORS AND OWNERSHIP
    println!("\n--- Vectors and Ownership ---");
    
    let mut numbers = vec![1, 2, 3];
    
    // Safe iteration - borrowing elements
    for num in &numbers {
        println!("Number: {}", num);
    }
    
    // Modifying during iteration
    for num in &mut numbers {
        *num *= 2;
    }
    println!("Doubled numbers: {:?}", numbers);
    
    // Moving out of vector (consuming)
    for num in numbers {
        println!("Consuming: {}", num);
    }
    // numbers is no longer valid here
    
    // PRACTICAL EXAMPLE: Text Processor
    println!("\n--- Practical Example: Text Processor ---");
    
    let text = String::from("  Hello, Rust World!  ");
    let processed = process_text(&text);
    println!("Original: '{}'", text);
    println!("Processed: '{}'", processed);
    
    // Word counting example
    let paragraph = "The quick brown fox jumps over the lazy dog. The dog was sleeping.";
    let word_count = count_words(paragraph);
    println!("Paragraph: {}", paragraph);
    println!("Word count: {}", word_count);
    
    // EXERCISES FOR YOU TO TRY:
    println!("\n--- Exercises ---");
    println!("1. Write a function that takes a String and returns the longest word");
    println!("2. Create a function that reverses a string slice and returns a new String");
    println!("3. Practice with vector ownership: write functions that modify vectors");
    println!("4. Implement a function that safely accesses vector elements by index");
    println!("5. Create a text analyzer that counts vowels, consonants, and spaces");
    
    println!("\n--- End of Lesson 3 ---");
    println!("Next lesson will cover: Structs, Enums, and Pattern Matching");
}

// FUNCTION DEFINITIONS

// Takes ownership of the string
fn take_ownership(s: String) {
    println!("I now own: {}", s);
} // s goes out of scope and is dropped

// Makes a copy of the integer
fn make_copy(x: i32) {
    println!("I have a copy: {}", x);
} // x goes out of scope, but it's just a copy

// Borrows a string without taking ownership
fn calculate_length(s: &String) -> usize {
    s.len()
} // s is a reference, so nothing is dropped

// Borrows a mutable string and modifies it
fn change_string(s: &mut String) {
    s.push_str(", World!");
}

// Returns a value (not a reference)
fn larger_number(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

// Returns a string slice (reference to part of the input)
fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // b' ' is byte literal for space
            return &s[0..i];
        }
    }
    
    &s[..] // Return entire string if no space found
}

// Takes ownership and returns ownership
fn add_word_and_return(mut words: Vec<&str>, new_word: &str) -> Vec<&str> {
    words.push(new_word);
    words
}

// Borrows to avoid taking ownership
fn count_items<T>(items: &Vec<T>) -> usize {
    items.len()
}

// Processes data without taking ownership
fn process_data(data: &String) {
    println!("Processing: {}", data);
}

// Text processing example
fn process_text(text: &str) -> String {
    text.trim().to_lowercase()
}

// Word counting example
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

// LESSON 3 SUMMARY:
// - Ownership: Each value has one owner, owner is responsible for cleanup
// - Move vs Copy: Some types move (String), others copy (i32)
// - Borrowing: Use & to create references without taking ownership
// - &T for immutable references, &mut T for mutable references
// - Borrowing rules: Many immutable OR one mutable reference at a time
// - String slices (&str) are references to parts of strings
// - References must be valid for their entire lifetime
// - Use clone() to create independent copies when needed
// - Dereference with * to access/modify values through references

// KEY CONCEPTS:
// Stack vs Heap:
// - Stack: Fast, fixed size, automatic cleanup (Copy types like i32)
// - Heap: Flexible size, manual cleanup (Move types like String)
// 
// When to use what:
// - Use & when you want to read data without taking ownership
// - Use &mut when you want to modify data without taking ownership
// - Use move (no &) when you want to transfer ownership
// - Use clone() when you need independent copies
// 
// Common patterns:
// - Functions that read: take &T
// - Functions that modify: take &mut T
// - Functions that consume: take T
// - Functions that create: return T
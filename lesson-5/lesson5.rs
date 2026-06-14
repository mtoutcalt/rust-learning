// Lesson 5: Error Handling with Result<T, E>
// To compile and run: rustc lesson5.rs && ./lesson5.exe

fn main() {
    println!("=== Lesson 5: Error Handling with Result ===");

    // RESULT - Like Option, but the "failure" case carries info
    // Result<T, E> is either Ok(value) or Err(error)
    println!("\n--- Result Basics ---");

    let good = divide(10, 2);
    let bad = divide(10, 0);

    println!("10 / 2 = {:?}", good); // Ok(5)
    println!("10 / 0 = {:?}", bad);  // Err("cannot divide by zero")

    // HANDLING RESULT WITH MATCH
    println!("\n--- Matching on Result ---");

    match divide(20, 4) {
        Ok(n) => println!("Success: {}", n),
        Err(e) => println!("Failed: {}", e),
    }

    // THE ? OPERATOR - Propagate errors with one character
    // If it's Err, return early. If it's Ok, unwrap the value.
    println!("\n--- The ? Operator ---");

    match halve_then_halve(40) {
        Ok(n) => println!("40 halved twice = {}", n),
        Err(e) => println!("Error: {}", e),
    }
    match halve_then_halve(1) {
        Ok(n) => println!("1 halved twice = {}", n),
        Err(e) => println!("Error: {}", e), // odd number fails
    }

    // UNWRAP AND EXPECT - Quick (but risky) ways to get the value
    // These PANIC if the Result is Err — use only when failure is a bug
    println!("\n--- unwrap / expect ---");

    let value = divide(8, 2).unwrap(); // OK here, we know it works
    println!("Unwrapped: {}", value);

    // unwrap_or gives a fallback instead of panicking
    let safe = divide(8, 0).unwrap_or(-1);
    println!("With fallback: {}", safe); // -1

    println!("\n--- End of Lesson 5 ---");
    println!("Next: Generics and Traits");
}

// Returns Ok with the result, or Err with a message
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// Returns Err if the number is odd
fn halve(n: i32) -> Result<i32, String> {
    if n % 2 != 0 {
        Err(format!("{} is odd, cannot halve evenly", n))
    } else {
        Ok(n / 2)
    }
}

// The ? operator: if halve returns Err, this function returns it immediately.
// Otherwise the Ok value is unwrapped and assigned.
fn halve_then_halve(n: i32) -> Result<i32, String> {
    let once = halve(n)?;   // early-return on Err
    let twice = halve(once)?;
    Ok(twice)
}

// LESSON 5 SUMMARY:
// - Result<T, E> is Ok(value) for success or Err(error) for failure
// - Like Option, but Err carries information about what went wrong
// - match handles both Ok and Err explicitly
// - ? propagates errors: returns Err early, or unwraps Ok
// - ? only works in functions that return Result (or Option)
// - unwrap()/expect() extract the value but PANIC on Err
// - unwrap_or(default) gives a fallback instead of panicking

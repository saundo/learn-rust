// Result Type Examples in Rust
// Result<T, E> is used for functions that can return an error

use std::fs::File;
use std::io::{self, Read, Write};
use std::num::ParseIntError;

// ============================================================================
// 1. Basic Result Usage
// ============================================================================

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// ============================================================================
// 2. Custom Error Types
// ============================================================================

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

// ============================================================================
// 3. Using ? Operator for Error Propagation
// ============================================================================

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// ============================================================================
// 4. Chaining Results with and_then
// ============================================================================

fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>().and_then(|n| Ok(n * 2))
}

// ============================================================================
// 5. Using map and map_err
// ============================================================================

fn parse_with_custom_error(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map(|n| n + 10)
        .map_err(|e| format!("Parse error: {}", e))
}

// ============================================================================
// 6. unwrap_or and unwrap_or_else
// ============================================================================

fn get_config_value(key: &str) -> Result<String, String> {
    if key == "username" {
        Ok(String::from("admin"))
    } else {
        Err(String::from("Key not found"))
    }
}

// ============================================================================
// 7. Combining Multiple Results
// ============================================================================

fn process_two_numbers(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let num_a = a.parse::<i32>()?;
    let num_b = b.parse::<i32>()?;
    Ok(num_a + num_b)
}

// ============================================================================
// 8. Pattern Matching on Result
// ============================================================================

fn handle_result_with_match(value: Result<i32, String>) -> i32 {
    match value {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error occurred: {}", e);
            0
        }
    }
}

// ============================================================================
// 9. Converting Between Result and Option
// ============================================================================

fn result_to_option(r: Result<i32, String>) -> Option<i32> {
    r.ok()
}

fn option_to_result(o: Option<i32>) -> Result<i32, String> {
    o.ok_or(String::from("Value was None"))
}

// ============================================================================
// 10. Using transpose with Option<Result>
// ============================================================================

fn parse_optional_number(s: Option<&str>) -> Result<Option<i32>, ParseIntError> {
    s.map(|s| s.parse::<i32>()).transpose()
}

// ============================================================================
// Main Function - Demonstrating All Examples
// ============================================================================

fn main() {
    println!("=== Result Type Examples ===\n");

    // Example 1: Basic Result
    println!("1. Basic Result:");
    match divide(10.0, 2.0) {
        Ok(result) => println!("   10 / 2 = {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    match divide(10.0, 0.0) {
        Ok(result) => println!("   10 / 0 = {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Example 2: Custom Error Types
    println!("\n2. Custom Error Types:");
    match safe_sqrt(16.0) {
        Ok(result) => println!("   sqrt(16) = {}", result),
        Err(e) => println!("   Error: {:?}", e),
    }
    match safe_sqrt(-4.0) {
        Ok(result) => println!("   sqrt(-4) = {}", result),
        Err(e) => println!("   Error: {:?}", e),
    }

    // Example 3: ? Operator
    println!("\n3. ? Operator (file reading):");
    match read_file_contents("nonexistent.txt") {
        Ok(contents) => println!("   File contents: {}", contents),
        Err(e) => println!("   Error reading file: {}", e),
    }

    // Example 4: and_then
    println!("\n4. Chaining with and_then:");
    match parse_and_double("21") {
        Ok(result) => println!("   Parsed and doubled: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Example 5: map and map_err
    println!("\n5. Using map and map_err:");
    match parse_with_custom_error("42") {
        Ok(result) => println!("   Parsed + 10: {}", result),
        Err(e) => println!("   {}", e),
    }
    match parse_with_custom_error("not_a_number") {
        Ok(result) => println!("   Parsed + 10: {}", result),
        Err(e) => println!("   {}", e),
    }

    // Example 6: unwrap_or and unwrap_or_else
    println!("\n6. unwrap_or and unwrap_or_else:");
    let username = get_config_value("username").unwrap_or(String::from("guest"));
    println!("   Username: {}", username);
    let missing = get_config_value("missing").unwrap_or_else(|e| {
        println!("   Using default due to: {}", e);
        String::from("default")
    });
    println!("   Missing key value: {}", missing);

    // Example 7: Combining Multiple Results
    println!("\n7. Combining Multiple Results:");
    match process_two_numbers("5", "10") {
        Ok(sum) => println!("   Sum: {}", sum),
        Err(e) => println!("   Error: {}", e),
    }

    // Example 8: Pattern Matching
    println!("\n8. Pattern Matching:");
    let value = handle_result_with_match(Ok(42));
    println!("   Handled value: {}", value);
    let error_value = handle_result_with_match(Err(String::from("Something went wrong")));
    println!("   Handled error value: {}", error_value);

    // Example 9: Result <-> Option Conversion
    println!("\n9. Result and Option Conversion:");
    let opt = result_to_option(Ok(100));
    println!("   Result to Option: {:?}", opt);
    let res = option_to_result(Some(200));
    println!("   Option to Result: {:?}", res);
    let res_none = option_to_result(None);
    println!("   None to Result: {:?}", res_none);

    // Example 10: transpose
    println!("\n10. Using transpose:");
    match parse_optional_number(Some("123")) {
        Ok(Some(n)) => println!("   Parsed optional: {}", n),
        Ok(None) => println!("   No value to parse"),
        Err(e) => println!("   Parse error: {}", e),
    }

    // Bonus: is_ok() and is_err()
    println!("\n11. Checking Result state:");
    let success: Result<i32, String> = Ok(42);
    println!("   Is Ok? {}", success.is_ok());
    println!("   Is Err? {}", success.is_err());
}

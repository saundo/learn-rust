// Option Type Examples in Rust
// Option<T> represents a value that might or might not exist

// ============================================================================
// 1. Basic Option Usage - Finding an item
// ============================================================================

fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else if id == 2 {
        Some(String::from("Bob"))
    } else {
        None  // User not found
    }
}

// ============================================================================
// 2. Using unwrap_or for Default Values
// ============================================================================

fn get_username(id: u32) -> String {
    find_user(id).unwrap_or(String::from("Guest"))
}

// ============================================================================
// 3. Pattern Matching on Option
// ============================================================================

fn greet_user(id: u32) {
    match find_user(id) {
        Some(name) => println!("Hello, {}!", name),
        None => println!("Hello, stranger!"),
    }
}

// ============================================================================
// 4. Using map to Transform Values
// ============================================================================

fn get_user_length(id: u32) -> Option<usize> {
    find_user(id).map(|name| name.len())
}

// ============================================================================
// 5. Using and_then to Chain Optional Operations
// ============================================================================

fn get_first_char(id: u32) -> Option<char> {
    find_user(id).and_then(|name| name.chars().next())
}

// ============================================================================
// 6. Using filter to Conditionally Keep Values
// ============================================================================

fn get_long_username(id: u32) -> Option<String> {
    find_user(id).filter(|name| name.len() > 4)
}

// ============================================================================
// Main Function - Demonstrating All Examples
// ============================================================================

fn main() {
    println!("=== Option Type Examples ===\n");

    // Example 1: Basic Option
    println!("1. Basic Option - Finding users:");
    println!("   User 1: {:?}", find_user(1));
    println!("   User 2: {:?}", find_user(2));
    println!("   User 99: {:?}", find_user(99));

    // Example 2: unwrap_or for defaults
    println!("\n2. Using unwrap_or for defaults:");
    println!("   Username for ID 1: {}", get_username(1));
    println!("   Username for ID 99: {}", get_username(99));

    // Example 3: Pattern matching
    println!("\n3. Pattern matching:");
    print!("   ");
    greet_user(1);
    print!("   ");
    greet_user(99);

    // Example 4: map to transform
    println!("\n4. Using map to transform:");
    println!("   Length of user 1's name: {:?}", get_user_length(1));
    println!("   Length of user 99's name: {:?}", get_user_length(99));

    // Example 5: and_then to chain
    println!("\n5. Using and_then to chain:");
    println!("   First char of user 1: {:?}", get_first_char(1));
    println!("   First char of user 99: {:?}", get_first_char(99));

    // Example 6: filter
    println!("\n6. Using filter:");
    println!("   Long username for ID 1: {:?}", get_long_username(1));
    println!("   Long username for ID 2: {:?}", get_long_username(2));

    // Bonus: is_some() and is_none()
    println!("\n7. Checking Option state:");
    let result = find_user(1);
    println!("   Has value? {}", result.is_some());
    println!("   Is empty? {}", result.is_none());

    // Bonus: if let syntax
    println!("\n8. Using if let:");
    if let Some(name) = find_user(1) {
        println!("   Found user: {}", name);
    } else {
        println!("   No user found");
    }
}

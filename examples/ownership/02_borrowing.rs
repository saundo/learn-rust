// Borrowing in Rust (Immutable References)
// Borrowing lets you reference a value without taking ownership

// ============================================================================
// 1. Basic Borrowing with &
// ============================================================================

fn calculate_length(s: &String) -> usize {
    s.len() // can read the value
    // s is NOT dropped here - we don't own it
}

fn demonstrate_basic_borrow() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // borrow with &
    
    println!("   String: {}", s1); // s1 is still valid!
    println!("   Length: {}", len);
}

// ============================================================================
// 2. Multiple Immutable Borrows are Allowed
// ============================================================================

fn print_twice(s: &String) {
    println!("   First: {}", s);
    println!("   Second: {}", s);
}

fn demonstrate_multiple_borrows() {
    let s = String::from("hello");
    
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    
    println!("   r1: {}", r1);
    println!("   r2: {}", r2);
    println!("   r3: {}", r3);
    println!("   original: {}", s);
}

// ============================================================================
// 3. References Must Always Be Valid
// ============================================================================

fn demonstrate_valid_references() {
    let s = String::from("hello");
    
    {
        let r = &s;
        println!("   Reference: {}", r);
    } // r goes out of scope
    
    println!("   Original still valid: {}", s);
}

// ============================================================================
// 4. Borrowing with Different Types
// ============================================================================

fn sum_vector(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}

fn demonstrate_vector_borrow() {
    let numbers = vec![1, 2, 3, 4, 5];
    let total = sum_vector(&numbers);
    
    println!("   Numbers: {:?}", numbers); // still valid
    println!("   Sum: {}", total);
}

// ============================================================================
// 5. Dereferencing with *
// ============================================================================

fn demonstrate_dereference() {
    let x = 5;
    let r = &x;
    
    println!("   Value of x: {}", x);
    println!("   Reference r: {:?}", r);
    println!("   Dereferenced *r: {}", *r);
    
    // Most of the time Rust auto-dereferences for you
    println!("   Auto-deref: {}", r);
}

// ============================================================================
// Main Function
// ============================================================================

fn main() {
    println!("=== Borrowing (Immutable References) ===\n");

    println!("1. Basic Borrowing:");
    demonstrate_basic_borrow();
    
    println!("\n2. Multiple Immutable Borrows:");
    demonstrate_multiple_borrows();
    
    println!("\n3. Reference Validity:");
    demonstrate_valid_references();
    
    println!("\n4. Borrowing Vectors:");
    demonstrate_vector_borrow();
    
    println!("\n5. Dereferencing:");
    demonstrate_dereference();
    
    println!("\n=== Key Takeaways ===");
    println!("• Use & to borrow without taking ownership");
    println!("• Can have multiple immutable borrows at once");
    println!("• References must always point to valid data");
    println!("• Original owner can still read the value");
    println!("• Use * to dereference (often automatic)");
}

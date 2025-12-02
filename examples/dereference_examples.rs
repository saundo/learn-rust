// Dereference Operator (*) in Rust
// Using * to access the value behind a reference

// ============================================================================
// 1. Basic Dereferencing
// ============================================================================

fn demonstrate_basic_deref() {
    let x = 5;
    let r = &x;  // r is a reference to x
    
    println!("   x = {}", x);
    println!("   r = {:p}", r);      // prints memory address
    println!("   *r = {}", *r);      // dereference to get value
    
    // Can use dereferenced value in expressions
    let sum = *r + 10;
    println!("   *r + 10 = {}", sum);
}

// ============================================================================
// 2. Modifying Through Mutable References
// ============================================================================

fn demonstrate_mutable_deref() {
    let mut x = 5;
    println!("   Before: x = {}", x);
    
    let r = &mut x;  // mutable reference
    *r = 10;  // dereference and assign new value
    println!("   After *r = 10: *r = {}", *r);
    
    *r += 5;  // dereference and modify
    println!("   After *r += 5: *r = {}", *r);
    
    // r is done being used, can access x again
    println!("   Final x = {}", x);
}

// ============================================================================
// 3. Dereferencing in Comparisons
// ============================================================================

fn demonstrate_deref_comparison() {
    let x = 5;
    let y = 5;
    let r = &x;
    
    // Need to dereference to compare values
    if *r == y {
        println!("   *r equals y");
    }
    
    // Without deref, comparing addresses (won't work as expected)
    // if r == &y { ... }
    
    println!("   *r == 5: {}", *r == 5);
    println!("   *r > 3: {}", *r > 3);
}

// ============================================================================
// 4. Dereferencing in Loops
// ============================================================================

fn demonstrate_deref_in_loops() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    println!("   Before: {:?}", numbers);
    
    // iter_mut gives us &mut i32
    for num in numbers.iter_mut() {
        *num *= 2;  // must deref to modify the value
    }
    
    println!("   After doubling: {:?}", numbers);
}

// ============================================================================
// 5. Multiple Levels of References
// ============================================================================

fn demonstrate_multiple_deref() {
    let x = 5;
    let r1 = &x;      // &i32
    let r2 = &r1;     // &&i32
    
    println!("   x = {}", x);
    println!("   *r1 = {}", *r1);      // one deref
    println!("   **r2 = {}", **r2);    // two derefs
    
    // Each * removes one level of reference
}

// ============================================================================
// 6. Auto-Dereferencing (When You Don't Need *)
// ============================================================================

fn demonstrate_auto_deref() {
    let s = String::from("hello");
    let r = &s;
    
    // Rust auto-dereferences for method calls
    println!("   Length (auto): {}", r.len());
    
    // These are equivalent:
    println!("   Length (manual): {}", (*r).len());
    
    // But for direct value access, you need *
    // let s2 = r;  // This gives you &String
    // let s2 = *r; // ERROR: can't move out of reference
}

// ============================================================================
// 7. Dereferencing with Pattern Matching
// ============================================================================

fn demonstrate_deref_pattern() {
    let x = 5;
    let r = &x;
    
    // Pattern matching with dereference
    match *r {
        5 => println!("   Matched: five"),
        _ => println!("   Matched: something else"),
    }
    
    // With mutable reference
    let mut y = 10;
    let r_mut = &mut y;
    
    match *r_mut {
        10 => {
            println!("   Found 10, changing to 20");
            *r_mut = 20;
        }
        _ => {}
    }
    
    println!("   y is now: {}", y);
}

// ============================================================================
// Main Function
// ============================================================================

fn main() {
    println!("=== Dereference Operator (*) ===\n");

    println!("1. Basic Dereferencing:");
    demonstrate_basic_deref();
    
    println!("\n2. Modifying Through Mutable References:");
    demonstrate_mutable_deref();
    
    println!("\n3. Dereferencing in Comparisons:");
    demonstrate_deref_comparison();
    
    println!("\n4. Dereferencing in Loops:");
    demonstrate_deref_in_loops();
    
    println!("\n5. Multiple Levels of References:");
    demonstrate_multiple_deref();
    
    println!("\n6. Auto-Dereferencing:");
    demonstrate_auto_deref();
    
    println!("\n7. Dereferencing with Pattern Matching:");
    demonstrate_deref_pattern();
    
    println!("\n=== Key Takeaways ===");
    println!("• * accesses the value behind a reference");
    println!("• Use *r to read the value");
    println!("• Use *r = value to modify through &mut");
    println!("• Multiple * for multiple reference levels");
    println!("• Rust auto-derefs for method calls");
    println!("• Need explicit * for comparisons and assignments");
}

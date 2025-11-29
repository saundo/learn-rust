// Mutable Borrowing in Rust
// &mut allows you to modify borrowed values, but with strict rules

// ============================================================================
// 1. Basic Mutable Borrowing
// ============================================================================

fn add_world(s: &mut String) {
    s.push_str(", world");
}

fn demonstrate_mutable_borrow() {
    let mut s = String::from("hello");
    println!("   Before: {}", s);
    
    add_world(&mut s);
    println!("   After: {}", s);
}

// ============================================================================
// 2. Only ONE Mutable Borrow at a Time
// ============================================================================

fn demonstrate_single_mutable_borrow() {
    let mut s = String::from("hello");
    
    let r1 = &mut s;
    r1.push_str(" world");
    println!("   r1: {}", r1);
    
    // Can't have r2 while r1 is still in use
    // let r2 = &mut s; // ERROR!
    // println!("{} {}", r1, r2);
    
    // But after r1 is done, we can create a new mutable borrow
    let r2 = &mut s;
    r2.push_str("!");
    println!("   r2: {}", r2);
}

// ============================================================================
// 3. Can't Mix Mutable and Immutable Borrows
// ============================================================================

fn demonstrate_borrow_rules() {
    let mut s = String::from("hello");
    
    let r1 = &s; // immutable borrow
    let r2 = &s; // another immutable borrow
    println!("   r1: {}, r2: {}", r1, r2);
    // r1 and r2 are no longer used after this
    
    let r3 = &mut s; // mutable borrow - OK now
    r3.push_str(" world");
    println!("   r3: {}", r3);
}

// ============================================================================
// 4. Modifying Vector Elements
// ============================================================================

fn double_values(v: &mut Vec<i32>) {
    for num in v.iter_mut() {
        *num *= 2;
    }
}

fn demonstrate_vector_mutation() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("   Before: {:?}", numbers);
    
    double_values(&mut numbers);
    println!("   After: {:?}", numbers);
}

// ============================================================================
// 5. Returning Mutable References
// ============================================================================

fn get_first_mut(v: &mut Vec<i32>) -> &mut i32 {
    &mut v[0]
}

fn demonstrate_return_mut_ref() {
    let mut numbers = vec![10, 20, 30];
    println!("   Before: {:?}", numbers);
    
    let first = get_first_mut(&mut numbers);
    *first = 99;
    
    println!("   After: {:?}", numbers);
}

// ============================================================================
// Main Function
// ============================================================================

fn main() {
    println!("=== Mutable Borrowing ===\n");

    println!("1. Basic Mutable Borrow:");
    demonstrate_mutable_borrow();
    
    println!("\n2. Only One Mutable Borrow:");
    demonstrate_single_mutable_borrow();
    
    println!("\n3. Borrow Rules (can't mix &mut with &):");
    demonstrate_borrow_rules();
    
    println!("\n4. Mutating Vector Elements:");
    demonstrate_vector_mutation();
    
    println!("\n5. Returning Mutable References:");
    demonstrate_return_mut_ref();
    
    println!("\n=== Key Takeaways ===");
    println!("• Use &mut to borrow and modify");
    println!("• Only ONE mutable borrow at a time");
    println!("• Can't have &mut and & at the same time");
    println!("• Prevents data races at compile time");
    println!("• Original value must be declared 'mut'");
}

// Lifetimes in Rust
// Ensuring references stay valid

// ============================================================================
// 1. The Problem Lifetimes Solve
// ============================================================================

// This won't compile - which reference should we return?
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// Solution: lifetime annotations tell Rust the relationship
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn demonstrate_basic_lifetime() {
    let string1 = String::from("long string");
    let string2 = String::from("short");
    
    let result = longest(&string1, &string2);
    println!("   Longest: {}", result);
}

// ============================================================================
// 2. Lifetime Annotations Explained
// ============================================================================

// 'a is a lifetime parameter
// It says: "the returned reference lives as long as the shortest input"
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    
    s
}

fn demonstrate_lifetime_annotation() {
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("   First word: {}", word);
    println!("   Full sentence: {}", sentence);
}

// ============================================================================
// 3. Multiple Lifetime Parameters
// ============================================================================

// Different lifetimes for different parameters
fn announce_and_return<'a, 'b>(announcement: &'a str, value: &'b str) -> &'a str {
    println!("   Announcement: {}", announcement);
    announcement // only returns 'a, not 'b
}

fn demonstrate_multiple_lifetimes() {
    let ann = String::from("Important!");
    let val = String::from("some value");
    
    let result = announce_and_return(&ann, &val);
    println!("   Returned: {}", result);
}

// ============================================================================
// 4. Lifetime Elision - When You Don't Need Annotations
// ============================================================================

// Rust can infer lifetimes in simple cases
fn first_char(s: &str) -> &str {
    &s[0..1]
}

// These are equivalent:
fn explicit_lifetime<'a>(s: &'a str) -> &'a str {
    &s[0..1]
}

fn demonstrate_elision() {
    let text = "hello";
    println!("   First char (inferred): {}", first_char(text));
    println!("   First char (explicit): {}", explicit_lifetime(text));
}

// ============================================================================
// 5. Structs with Lifetimes
// ============================================================================

// Struct that holds a reference needs a lifetime
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    fn get_text(&self) -> &str {
        self.text
    }
    
    fn announce(&self, announcement: &str) -> &str {
        println!("   {}", announcement);
        self.text
    }
}

fn demonstrate_struct_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    let excerpt = Excerpt {
        text: first_sentence,
    };
    
    println!("   Excerpt: {}", excerpt.get_text());
    excerpt.announce("From Moby Dick:");
}

// ============================================================================
// 6. Static Lifetime
// ============================================================================

// 'static means the reference lives for the entire program
fn demonstrate_static() {
    let s: &'static str = "I live forever";
    println!("   Static string: {}", s);
    
    // String literals have 'static lifetime
    let literal = "also static";
    println!("   Literal: {}", literal);
}

// ============================================================================
// Main Function
// ============================================================================

fn main() {
    println!("=== Lifetimes ===\n");

    println!("1. Basic Lifetime:");
    demonstrate_basic_lifetime();
    
    println!("\n2. Lifetime Annotations:");
    demonstrate_lifetime_annotation();
    
    println!("\n3. Multiple Lifetimes:");
    demonstrate_multiple_lifetimes();
    
    println!("\n4. Lifetime Elision:");
    demonstrate_elision();
    
    println!("\n5. Structs with Lifetimes:");
    demonstrate_struct_lifetime();
    
    println!("\n6. Static Lifetime:");
    demonstrate_static();
    
    println!("\n=== Key Takeaways ===");
    println!("• Lifetimes ensure references stay valid");
    println!("• 'a is a lifetime parameter (like a generic)");
    println!("• Returned reference can't outlive inputs");
    println!("• Rust often infers lifetimes (elision)");
    println!("• Structs with references need lifetimes");
    println!("• 'static lives for entire program");
}

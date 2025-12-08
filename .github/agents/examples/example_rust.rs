// Example Rust code for testing the Code Review Agent
//
// This file contains some intentional issues that the agent should detect:
// - Unsafe code blocks
// - Potential memory issues
// - Non-idiomatic Rust code
// - Missing error handling

use std::fs::File;
use std::io::Read;

// Unsafe code that should be reviewed
pub fn get_string_from_ptr(ptr: *const u8, len: usize) -> String {
    unsafe {
        // This is dangerous and should be reviewed
        let slice = std::slice::from_raw_parts(ptr, len);
        String::from_utf8_unchecked(slice.to_vec())
    }
}

// Missing error handling - uses unwrap()
pub fn read_file_content(filename: &str) -> String {
    let mut file = File::open(filename).unwrap(); // Can panic!
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); // Can panic!
    contents
}

// Not using idiomatic Result type
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero"); // Should return Result instead
    }
    a / b
}

// Inefficient - creates unnecessary copies
pub fn process_vec(data: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for item in data {
        result.push(item * 2);
    }
    result
}

// Should use Iterator trait instead
pub fn sum_numbers(numbers: Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in numbers {
        sum += num;
    }
    sum
}

// Memory leak potential with Box::into_raw
pub fn create_leaked_box() -> *mut i32 {
    let boxed = Box::new(42);
    Box::into_raw(boxed) // Leaks memory unless properly freed
}

fn main() {
    println!("Example Rust code with issues");
}

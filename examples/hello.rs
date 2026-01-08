// Attributes - metadata for compiler
// Tells a compiler to ignore whatever variable or functions that are not used
// Without this attribute, code will show warnings and will be hard to analyze. Which suppresses warnings on unused vars and funcs
#![allow(unused)]

// Main function - main() is the entry point of Rust program
// When we bash on terminal as "cargo run", it looks for main() function to compile and process
// It will search for main.rs file by default, so to run this example.rs, we need to add 'cargo run --example hello' as no adding .rs file to it.
fn main() {
    // Functions with exclamation marks are not function, it's called macros. (println! is a macro.)
    // Macros in Rust generate code at compile time and are invoked with an exclamation mark (!)
    // println! is a macro that prints text to the console
    println!("Hello, world!");
}

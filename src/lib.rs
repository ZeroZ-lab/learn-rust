// Declare the module found in src/my_module.rs
pub mod my_module; 
pub mod utils;

// Define the trait needed for the procedural macro example
pub trait HelloMacro {
    fn hello_macro();
}
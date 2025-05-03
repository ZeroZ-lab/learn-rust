// Use the greet function from the my_module module in our library crate (learn_rust)
use learn_rust::my_module::greet;
use learn_rust::utils::pad_left;

fn main() {
    println!("Calling module function from library...");
    // Call the public function using the imported path
    greet();

    let padded_string = pad_left("Hello, world!", ' ', 10);
    println!("Padded string: {}", padded_string);

    // This still wouldn't work because private_function is private within my_module:
    // learn_rust::my_module::private_function();
} 
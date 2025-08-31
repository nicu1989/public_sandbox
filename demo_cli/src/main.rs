/// Main function
///
/// # Examples
///
/// ```
/// cargo run -- Demo
/// ```
///
/// # Arguments
///
/// * `name` - The name to greet
///
/// # Returns
///
/// The greeting message
fn main() {
    let name = std::env::args().nth(1).unwrap_or_else(|| "World".to_string());
    let message = crates_io_demo_lib_nicu1989::greet(&name);
    println!("{}", message);
}


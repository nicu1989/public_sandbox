//! Minimal lib for demonstrating crates.io publishing.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Returns a "Hello, {name}!"
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_greets() {
        assert_eq!(greet("World"), "Hello, World!");
    }
}


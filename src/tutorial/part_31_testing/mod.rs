mod circle;
mod math_functions;
mod person;

// the convention in rust programming is to have
// tests in the same file as the code being tested.
mod test {
    #[cfg(test)]
    #[test]
    #[ignore]
    fn test_to_be_ignored() {
        // to run ignored tests use: cargo test -- --ignored
    }
}

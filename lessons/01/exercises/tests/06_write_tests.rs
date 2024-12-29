//! Run this file with `cargo test --test 06_write_tests`.

/// This function implements a string sanitization logic that should uphold the following
/// properties:
/// - After sanitization, the result must not end with the character `x`
/// - After sanitization, the result must not end with the character `o`
/// - After sanitization, the result must not end with the string `.exe`
///
/// The function assumes that the input to the function only consists of lower and uppercase
/// characters from the English alphabet and digits 0-9.
///
/// The implementation contains some bugs.
///
/// Your task is to write a set (at least 8) of unit tests, use them to find (at least 2) bugs in
/// this function and then fix the function.
/// original solution
fn _old_sanitize(input: &str) -> &str {
    // Remove all x from the end of the string
    let input = input.trim_end_matches("x");
    // Remove all o from the end of the string
    let mut input = input.trim_end_matches("o");

    // Remove .exe from the end
    if input.ends_with(".exe") {
        input = &input[0..input.len() - 4];
    }

    input
}

/// fixed solution
fn sanitize(input: &str) -> &str {
    if input.len() == 0 {
        return input;
    }

    let mut result = input;
    while result.ends_with("x") || result.ends_with("o") || result.ends_with(".exe") {
        if result.ends_with(".exe") {
            result = &result[0..result.len() - 4];
        } else {
            result = &result[0..result.len() - 1];
        }
    }

    result
}

/// DONE: write tests for the `sanitize` function
///
/// Bonus: can you find any bugs using the [proptest](https://proptest-rs.github.io/proptest/intro.html)
/// crate?
/// Note that you will probably need to run `cargo test` with the `PROPTEST_DISABLE_FAILURE_PERSISTENCE=1`
/// environment variable to make proptest work for tests stored in the `tests` directory.
#[cfg(test)]
mod tests {
    use super::sanitize;
    use proptest::proptest;

    #[test]
    fn sanitize_empty() {
        assert!(sanitize("") == "");
    }

    #[test]
    fn sanitize_basic() {
        assert!(sanitize("windows.exe") == "windows");
        assert!(sanitize("io") == "i");
        assert!(sanitize("matrix") == "matri");
    }

    #[test]
    fn sanitize_multiple() {
        assert!(sanitize("windowsx.exe") == "windows");
        assert!(sanitize("ioo") == "i");
        assert!(sanitize("matrixx") == "matri");
    }

    #[test]
    fn sanitize_complex() {
        assert!(sanitize("windowsx.exeo") == "windows");
        assert!(sanitize("ioox") == "i");
        assert!(sanitize("matrixx.exe") == "matri");
    }


    // bug 1, the cleaning happens sequentially 
    // one after the other
    // x -> o -> .exe 
    // what happens if the occurence of the suffix is jumbled 

    // bug 2 
    // multiple occurences of the same suffix will also not help 
    // .exe.exe will result in only one of the suffixes being trimmed 

    // bug 3
    // the function does not handle the case where the input is empty

    // bug 4
    // the function assumes alphanumeric inputs 

    proptest! {
        #[test]
        fn proptest_checks(s in "\\PC*") {
            assert!(!sanitize(&s).ends_with("o"));
            assert!(!sanitize(&s).ends_with("x"));
            assert!(!sanitize(&s).ends_with(".exe"));
        }
    }
}

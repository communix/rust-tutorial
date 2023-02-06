//! # Rust example code 0001 - Find first word
//! Jay Kim (changjae.kim@gmail.com)
//! 2023 version 0.0.1

/// write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
/// If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.

fn main() {
    let input_string: String = String::from("Hello world. good to see you.");
    let input_word: String = String::from("Verification");
    let first_output: &str = find_first_word(&input_string[..]);
    let second_output: &str = find_first_word(& input_word[..]);
    println!("First word in the /{}/ is /{}/", input_string, first_output);
    println!("First word in the /{}/ is /{}/", input_word, second_output);
}

// Generally, use &str instead of the String in the furnction parameter and return.
// it's more generic because it support String type and String literal (&str)
fn find_first_word(in_str: &str) -> &str { // &str is String slice
    let string_byte: &[u8] = in_str.as_bytes(); // as_bytes() function is used to make char (u8) array
    for (i, &byte) in string_byte.iter().enumerate() { // enumerate() return a tuple of index and data
        if byte == b' ' {
            return &in_str[0..i];
        }
    }
    return &in_str[..]
}
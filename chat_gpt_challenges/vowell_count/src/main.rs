// Coding challenge created by chat GPT on Chapter 4

use std::io;

fn main() {
    // Read input from the user
    let mut input = String::new();
    println!("Enter a string:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Process the string
    let string_length = input.trim().len();
    let vowel_count = count_vowels(&input);

    // Print the results
    println!("Length of the string: {}", string_length);
    println!("Number of vowels: {}", vowel_count);
}

fn count_vowels(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut count = 0;
    let vowels = [b'a', b'e', b'i', b'o', b'u'];

    for (_, &c) in bytes.iter().enumerate() {
        if vowels.contains(&c) {
            count += 1;
        };
    }

    return count;
}

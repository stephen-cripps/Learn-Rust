/*
Challenge: Write a Rust program that takes a sentence as input and does the following:

    Split the sentence into individual words.
    Create a vector to store these words.
    Count the frequency of each word in the sentence.
    Print out the unique words and their respective frequencies.
*/

use std::{collections::HashMap, io::stdin};

fn main() {
    println!("Please input a sentence");

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to Read Line");

    let mut count = HashMap::new();

    for word in input.split_whitespace() {
        let val = count.entry(word).or_insert(0);
        *val += 1;
    }

    println!("{:?}", count)
}

/*
Challenge: Implement a Custom Filter Function

Write a Rust function called custom_filter that takes a slice of integers and a closure as arguments. The function should return a new Vec<i32> containing only the elements from the input slice for which the closure returns true.

Here's a function signature to get you started:

rust

fn custom_filter<F>(numbers: &[i32], predicate: F) -> Vec<i32>
where
    F: Fn(i32) -> bool,
{
    // Your code here
}

Here are some additional requirements and hints:

    You'll need to use iterators to iterate over the elements of the input slice.
    The closure predicate should accept an integer and return a boolean value.
    Use the closure to filter the elements from numbers and create a new Vec<i32> containing the elements that satisfy the predicate.
    Test your custom_filter function with different predicates to ensure it works as expected.

Here's an example of how you might use the custom_filter function:

rust

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let filtered_numbers = custom_filter(&numbers, |x| x % 2 == 0);

    println!("Filtered Numbers: {:?}", filtered_numbers);
}

The above code should print [2, 4, 6, 8, 10], which are the even numbers from the input slice.
*/

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let filtered_numbers = custom_filter(&numbers, |x| x % 2 == 0);

    println!("Filtered Numbers: {:?}", filtered_numbers);
}

fn custom_filter<F>(numbers: &[i32], predicate: F) -> Vec<i32>
where
    F: Fn(i32) -> bool,
{
    let mut result = Vec::new();

    for &number in numbers {
        if predicate(number) {
            result.push(number)
        }
    }

    result
}

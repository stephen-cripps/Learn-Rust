/*
Challenge: Create a Generic Stack

Write a generic Rust struct called Stack<T> that represents a stack data structure. The stack should support the following operations:

    new() - Create a new empty stack.
    push(item: T) - Push an item onto the stack.
    pop() - Pop and return the top item from the stack (if it's not empty).
    peek() - Return a reference to the top item without removing it (if the stack is not empty).
    is_empty() - Check if the stack is empty.

 */

use std::fmt::Display;

const MAX_SIZE: usize = 100;

struct Stack<T>
where
    T: Copy,
{
    // Your fields here
    value: [Option<T>; MAX_SIZE],
    size: usize,
}

impl<T> Stack<T>
where
    T: Copy,
{
    // Constructor method
    fn new() -> Self {
        let arr: [Option<T>; MAX_SIZE] = [None; MAX_SIZE];

        Stack {
            value: arr,
            size: 0,
        }
    }

    // Push an item onto the stack
    fn push(&mut self, item: T) -> Result<(), String> {
        if self.size == MAX_SIZE {
            return Err("Cannot add any more items to stack".to_string());
        }

        self.value[self.size] = Some(item);
        self.size += 1;

        Result::Ok(())
    }

    // Pop and return the top item from the stack
    fn pop(&mut self) -> Result<Option<T>, String> {
        if self.size == 0 {
            return Err("Stack is empty".to_string());
        }
        self.size -= 1;
        let res = self.value[self.size];
        self.value[self.size] = None;

        Result::Ok(res)
    }

    // Return a reference to the top item
    fn peek(&self) -> &Option<T> {
        &self.value[self.size - 1]
    }

    // Check if the stack is empty
    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl Display for Stack<u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values: Vec<u32> = self.value.into_iter().filter_map(|x| x).collect();

        write!(f, "{}", format!("{:?}", values))
    }
}

fn main() {
    let mut stack = Stack::<u32>::new();
    println!("Initial Stack: {}", stack);

    stack.push(1).unwrap();
    stack.push(2).unwrap();
    stack.push(3).unwrap();
    println!("Pushed Stack: {}", stack);

    println!("Popped Value: {}", stack.pop().unwrap().unwrap());
    println!("Popped Stack: {}", stack);

    println!("Peeked Value: {}", stack.peek().unwrap());
    println!("Peeked Stack: {}", stack);

    println!("Empty: {}", stack.is_empty());
    stack.pop().unwrap();
    stack.pop().unwrap();
    println!("Empty: {}", stack.is_empty());
}

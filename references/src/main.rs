fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let len2 = calculate_length_2(&s2);

    println!("The length of '{}' is {}.", s2, len2);

    let mut s3 = String::from("hello");

    mutate(&mut s3);

    // S3 has been mutated by the method that borrowed it
    println!("{}", s2);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    // Without references, if we want to keep the original variable we have to return it from the method, as the value has moved from s1 to s
    (s, length)
}

fn calculate_length_2(s: &String) -> usize {
    // With the reference, s here does not own the data, it has 'borrowed' the data from s2
    s.len()
}

fn mutate(s: &mut String) {
    //References are immutable by default, but can be marked as mutable

    s.push_str(", world!");
}

fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    // println!("{}", s); This won't compile because s has moved

    let x = 5;

    makes_copy(x);

    println!("{}", x); // This is fine because x is stored on the stack, so is copied instead of moved

    let t = String::from("Hi");

    let u = takes_and_gives_back(t);

    println!("{}", u); // This is fine because the value moves from t => fn => u
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

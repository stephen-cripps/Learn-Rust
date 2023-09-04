fn main() {
    vectors();
    strings();
}

fn vectors() {
    let mut v = vec![1, 2, 3];

    v.push(4);

    // Gets Value
    let third = &v[2];
    println!("The third element is {third}");

    // Gets Option
    if let Some(el) = v.get(2) {
        println!("The third element is {el}");
    } else {
        println!("There is no third element")
    }

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }
}

fn strings() {
    // This generates a string from a string literal
    let mut s = String::new();
    s = "initial contents".to_string();

    // This is equivalent
    let mut s2 = String::from("initial contents");

    s2 = "test".to_string();

    s.push_str(" concatenated text");

    // strings can't be indexed directly and need converting to chars first
    let chars = s.chars();

    for c in chars {
        println!("{c}");
    }
}

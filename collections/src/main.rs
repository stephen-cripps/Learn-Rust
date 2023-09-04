use std::collections::HashMap;

fn main() {
    // vectors();
    // strings();
    hashMaps();
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

fn hash_maps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 4);
    scores.insert(String::from("Red"), 5);

    let team_name = String::from("Blue");

    // use get to pull out scores as an option then unwrap
    let blue_score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{blue_score}");

    // Overwrites score
    scores.insert(String::from("Blue"), 6);

    // Adds score only if key is empty
    scores.entry(String::from("Blue")).or_insert(10);
    scores.entry(String::from("Yellow")).or_insert(10);

    // iterate over scores
    for (key, value) in scores {
        println!("{key}: {value}")
    }

    // Updating entry based on old value
    let text = "This is a sentence of great interest and a bit of repetition";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map)
}

fn main() {
    // Assigning Loop Results
    let mut counter = 0;

    let result = loop {
        println!("Counter: {counter}");
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result: {result}");

    // Loop Labels
    let mut count = 0;

    'counting_up: loop {
        let mut remaining = 10;
        println!("count: {count}");

        loop {
            println!("remaining: {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1
        }

        count += 1
    }

    println!("End Count: {count}");

    // while
    let mut number = 3;

    while number != 0 {
        println!("{number}..");
        number -= 1;
    }

    println!("Lift off!!");

    // for
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // for with range
    for number in (1..4).rev() {
        println!("{number}..");
    }
    println!("Lift off!!");
}

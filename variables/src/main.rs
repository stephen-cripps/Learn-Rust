fn main() {
    // Mutable variables
    let mut x = 5;

    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // Shadowed Variables
    let y = 5;

    println!("The value of y is {y}");

    let y = y + 1;

    {
        let y = y*y;
        println!("The scoped value of y is {y}");
    }

    println!("The value of y is {y}");
}

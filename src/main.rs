/*exercise-1 run hello world*/
use std::any::type_name_of_val;

fn hello_world() {
    print!("Hello,");
    println!("world!");
}

/* exercise-2 print variables */
fn print_variables() {
    let x = 5;
    let y = 10;

    /* example with position templates */
    println!("x: {}, y: {}", x, y);
    let name = "Aman Singh";
    let age = 21;

    /* example with position index */
    println!("Name: {}, Age: {} is {0}", name, age);

    /* example with direct passing of variable*/
    println!("Name {name}")
}

/*exercise-3 rust variables */
fn rust_variables() {
    let x = 5;
    let y = 10;
    let z = x + y;
    println!("The value of z is: {}", z);

    /*variables are immutable by default*/
    // x = 10; this will throw an error

    let mut a = 5; // mutable variable declaration
    a = 10;
    println!("The value of a is: {}", a);

    const NAME: &str = "Aman Singh"; // constant declaration
    println!("Constant: {}", NAME);
}

/*exercise-4 rust type casting */
fn type_casting() {
    let x: i32 = 5;
    let y: f64 = x as f64;
    println!("The value of y is: {}", y);
    //     print type of variable
    println!("Type of y is: {}", type_name_of_val(&y));

    let my_char = 'A';
    let my_int = my_char as i32;
    println!("The value of my_int is: {}", my_int);
}

/*exercise-4 rust arithmetic*/
fn arithmetic() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let dividend = 21;
    let divisor = 8;
    let quotient = dividend / divisor; // integer division will return quotient
    let full_division = dividend as f32 / divisor as f32; // float division will return full division
    let remainder = dividend % divisor;

    println!("Sum: {sum}, Difference: {difference}, Product: {product}, Quotient: {quotient}, Remainder: {remainder}, Full Division {full_division}");
}

/* exercise-5 conditions */
fn conditions() {
    let number = 3;
    if number < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is greater than or equal to 5");
    }
}

/*exercise-6 loops*/
fn loops() {
    let mut counter = 0;
    /* runs indefinitely until something breaks it */
    loop {
        counter += 1;
        println!("Counter: {}", counter);
        if counter == 10 { break; };
    }

    let mut counter = 0;
    while counter < 10 {
        counter += 1;
        println!("Counter: {}", counter);
    }

    for number in 1..11 {
        println!("Number: {}", number);
    }
}

/* exercise-7 pattern matching */
fn pattern_matching() {
    let day = "Monday";
    match day {
        "Monday" => println!("It's Monday"),
        "Tuesday" => println!("It's Tuesday"),
        "Wednesday" => println!("It's Wednesday"),
        _ => println!("Not in here")
    }

    enum Direction {
        Up,
        Down,
        Left,
        Right
    }

    let player_direction = Direction::Up;

    match player_direction {
        Direction::Up => { println!("Moving Up")}
        Direction::Down => {println!("Moving Down")}
        Direction::Left => {println!("Moving Left")}
        Direction::Right => {println!("Moving Right")}
    }

    let test_option = Some(5);
    match test_option {
        Some(value) => { println!("It's five, {value}") }
        None => { println!("It's nothing") }
    }

    let my_result: Result<i32, &str> = Ok(100);

    // use of match expression to match Result type
    match my_result {
        Ok(value) => println!("The result is {}", value),
        Err(error) => println!("The error message is {}", error),
    }

    let my_option: Option<i32> = Some(222);

    // use of if let expression on the Option type
    if let Some(value) = my_option {
        println!("The option has a value of {}", value);
    } else {
        println!("The option has no value");
    }
}

fn main() {
    hello_world();
    print_variables();
    rust_variables();
    type_casting();
    arithmetic();
    conditions();
    loops();
    pattern_matching()
}

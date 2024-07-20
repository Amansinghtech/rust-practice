/*exercise-1 run hello world*/
use std::any::type_name_of_val;
use std::ops::Index;

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

fn rust_arrays() {
//     array declaration
    let nums = [1, 2, 3, 4, 5];
    let nums2: [i8; 10] = [1,2,3,4,5,6,7,8,9,10];
    let nums3 = [1; 5]; // [1, 1, 1, 1, 1] array with 5 elements with value 1
    // an array without data type
    let a = [5, 4, 3, 2, 1];

    // an array with data type and size
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // an array with default values
    let c = [3; 5];

    println!("a = {a:?}");
    println!("b = {:?}", b);
    println!("c = {:?}", c);

//     access array elements
    let first = a[0];
    let second = a[1];
    println!("First: {first}, Second: {second}");

//     get array length
    let length = a.len();
    println!("Length of a: {length}");

//     change array elements
    let mut d = [1, 2, 3, 4, 5];
    d[0] = 10;
    println!("d = {:?}", d);

//     slicing array
    let after_three = &d[3..];
    let before_three = &d[..3];
    let between_one_four = &d[1..];
    println!("After three: {after_three:?}");
    println!("Before three: {before_three:?}");
    println!("Between one and four: {between_one_four:?}");
}

fn mutable_slice_array() {
    // mutable array
    let mut colors = ["red", "green", "yellow", "white"];

    println!("array = {:?}", colors);

    // mutable slice
    let sliced_colors = &mut colors[1..3];

    println!("original slice = {:?}", sliced_colors);

    // change the value of the original slice at the first index
    sliced_colors[1] = "purple";

    println!("changed slice = {:?}", sliced_colors);
}

fn rust_tuples() {
    let random_tuple = ("Hello", 200, 3.14);

    // accessing tuple element at index 0
    println!("Value at Index 0 = {}", random_tuple.0);

    // accessing tuple element at index 1
    println!("Value at Index 1 = {}", random_tuple.1);

    // accessing tuple element at index 2
    println!("Value at Index 2 = {}", random_tuple.2);

//     destructure tuple
    let (name, value, pi) = random_tuple;
    println!("Name: {name}, Value: {value}, Pi: {pi}");
}

fn rust_structures() {
    struct Person {
        name: String,
        age: i32,
        height: f32,
    }

    let person = Person {
        name: "Aman Singh".to_string(),
        age: 21,
        height: 31.5,
    };

    println!("Name: {0}, Age: {1}, Height: {2}", person.name, person.age, person.height);
}

fn rust_closure() {
//     same as anonymous functions or lambda functions
    let add = |a: i32, b: i32| a + b;
    let result = add(5, 10);
    println!("Result: {result}");

    // define a multi-line closure
    let squared_sum = |x: i32, y: i32| {

        // find the sum of two parameters
        let mut sum: i32 = x + y;

        // find the squared value of the sum
        let mut result: i32 = sum * sum;

        return result;
    };

    // call the closure
    let result = squared_sum(5, 3);

    println!("Result = {}", result);

}

fn mutable_closure() {
    let mut word = String::from("Hello");

    // mutable closure
    let mut print_str = || {
        // value of word is changed here
        word.push_str(" World!");
        println!("word = {}", word);
    };

    // cannot immutable borrow because the variable is borrowed as mutable inside the closure
    // println!("length of word = {}", word.len());

    print_str();

    // can immutable borrow because the closure has been already used
    println!("length of word = {}", word.len());
}

fn immutable_closure() {
    let word = String::from("Hello");

    // immutable closure
    let print_str = || {
        // word variable is moved to a new variable
        let new_word = word;
        println!("word = {}", new_word);
    };

    print_str();

    // cannot immutable borrow because word variable has moved inside closure
    // println!("length of word = {}", word.len());
}

fn rust_vectors() {
// a vector is a dynamic data structure that store data in heap. It can store multiple values of same data type
    let mut numbers = vec![1, 2, 3];
    println!("Numbers: {:?}", numbers);

    // capacity of the vector
    let capacity = numbers.capacity();
    println!("Capacity: {capacity}");

    // add an element to the vector
    numbers.push(4);
    numbers.push(5);
    numbers.push(6);
    numbers.push(7);
    println!("Numbers: {:?}", numbers);

    // remove an element from the vector
    numbers.pop();
    println!("Numbers: {:?}", numbers);

    // get the length of the vector
    let length = numbers.len();
    println!("Length: {length}");

    // get the capacity of the vector
    let capacity = numbers.capacity();
    println!("Capacity: {capacity}");

    // check if the vector is empty
    let is_empty = numbers.is_empty();
    println!("Is Empty: {is_empty}");

    // get the first element of the vector
    let first = numbers[0];
    println!("First: {first}");

    // get the last element of the vector
    let last = numbers[numbers.len() - 1];
    println!("Last: {last}");

    // get the element at index 2
    let second = numbers.get(2);
    println!("Second: {second:?}");

    // loop through the vector
    for number in numbers.iter() {
        println!("Number: {number}");
    }

    // loop through the vector and mutate the values
    for number in numbers.iter_mut() {
        *number *= 2;
    }
    println!("Numbers: {:?}", numbers);

}

fn main() {
    hello_world();
    print_variables();
    rust_variables();
    type_casting();
    arithmetic();
    conditions();
    loops();
    pattern_matching();
    rust_arrays();
    mutable_slice_array();
    rust_tuples();
    rust_structures();
    rust_closure();
    mutable_closure();
    immutable_closure();
    rust_vectors();
}


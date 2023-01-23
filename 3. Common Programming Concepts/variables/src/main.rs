use std::any;
use std::io;

fn main() {
    mutable();
    shadowing();
    num_operations();
    bools();
    chars();
    tuples();
    arrays();
    access_array_example();
    another_function();
    another_function2(42);
    print_labeled_measurement(5, 'h');
    let x = five();

    println!("The value of x is: {x}");

    println!("{}", plus_one(5));
}

fn mutable() {
    let mut x = 5; // without mut, this variable is not mutable...
    println!("The value of x is: {x}");
    x = 6; // and will show an "assign to immutable variable error" here
    println!("The value of x is {x}");
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn num_operations() {
    // addition
    let sum = 5 + 10; // type is i32
    print_type_of(&sum);

    // subtraction
    let difference = 95.5 - 4.3; //f64
    print_type_of(&difference);

    // multiplication
    let product = 4 * 30; // type is i32
    print_type_of(&product);

    // division
    let quotient = 56.7 / 32.2; // type is f64
    let truncated = -5 / 3; // result is -1 ; type is i32
    print_type_of(&quotient);
    print_type_of(&truncated);

    // remainder
    let remainder = 43 % 5; // type is i32
    print_type_of(&remainder);
}

fn bools() {
    let t = true; // bool
    print_type_of(&t);

    let f: bool = false; // with explicit type annotations; type is bool
    print_type_of(&f);
}

fn chars() {
    let c = 'z'; // char
    print_type_of(&c);

    let z: char = 'ℤ'; // with explicit type annotation; type is char
    print_type_of(&z);

    let heart_eyed_cat = '👀'; // char
    print_type_of(&heart_eyed_cat);
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1); // type is (i32, f64, u8)

    let (x, y, z) = tup; // deconstructed tuple

    println!("The value of y is: {y}");
    print_type_of(&tup);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0; // results in 500

    let six_point_four = x.1; // results in 64

    let one = x.2; // results in 1
}

fn arrays() {
    let a = [1, 2, 3, 4, 5]; // type is [i32; 5] (type, length)
    print_type_of(&a);

    let b: [i32; 5] = [1, 2, 3, 4, 5]; // type is [i32; 5]
    print_type_of(&b);

    let c = [3; 5]; // results in: [3, 3, 3, 3, 3]

    let first = a[0]; // results in 1
    let second = a[1]; // results in 2
}

fn access_array_example() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    // a index out of bounds will result in a runtime error.
    // The code will still compile! Compared to C, this is quite a pleasure as in C we will access invalid memory.

    println!("The value of the element at index {index} is: {element}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5   // compared to other languages, rusts return value is implicitly the last expression of a function block
}

fn plus_one(x: i32) -> i32 {
    x + 1 // a semicolon here would create a statement and run into a compile error!
}

fn print_type_of<T>(_: &T) {
    // function which prints out the type of a variable
    println!("{}", any::type_name::<T>());
}

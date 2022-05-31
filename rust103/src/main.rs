// common programming concepts
use std::io;

fn main() {
    println!("-----------variables and mutability-----------");
    let x = 10;
    println!("The value of x is {}", x);
    // cannot assign twice to immutable variable
    // x = 12;
    // println!("The value of x is {}", x);

    // make them mutable by adding mut in front of the variable name
    let mut y = 20;
    println!("The value of y is {}", y);
    y = 22;
    println!("The value of y is {}", y);
    
    // constant
    // bound to a name and are not allowed to change
    // an't allow to use mut
    // the type of the value must be annotated
    // may be set only to constant expression
    const _ONE_HOUR_IN_SECOND: u32 = 60 * 60 * 1;

    // shadowing
    // declare a new variable with the same name as a previous variable
    {
        let x = 30;
        println!("The value of x is {}", x);

        let y = y - 10;
        println!("The value of y is {}", y);

    }
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces);

    // we're not allowed to mutate a variable's type
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("The value of spaces is {}", spaces);

    println!("-----------datatypes-----------");
    
    // rust is a statically typed language: it must know the types of all variables at compile time
    // rust's defaults "i32" are generally good places to start: if we unsure which type to use
    // scalar types
    // represets a single value: int, float, bool, char
    // int : a numbter without a fractional component.
    let _a: i32 = 2;
    let _b: i64;
    // signed and unsigned refer to whether it's possible for the number to be negative

    // float rust defualt : f64
    let _x = 2.;      // f64
    let _y: f32 = 3.2; // f32

    // numeric operation
    let _sum = 5 + 10;        // addition
    let _difference = 10 - 5; // substraction
    let _product = 5 * 10;    // multiplication
    let _floored = 2 / 3;     // division
    let _remainder = 45 % 3;  // remainder

    // bool
    let t = true;
    let f: bool = false; 
    println!{"Boolean value {}, {}", t, f}

    // char
    let c = 'c';
    let z = 'Z';
    let heart_eyes_cat = '😻';
    let hao = '好';
    println!("Character type: {}, {}, {}, {}", c, z, heart_eyes_cat, hao);

    // compound types
    // tuple grouping together a number of value with "variety" of types into one compound type
    let tup: (i32, f64, char, bool) = (3, 1.3, '💩', true);
    println!("Indexing tuple {}", tup.2);
    // unpacking = destructing
    let (_a, _b, _c, _d) = tup;

    // array: every element of an array must have the same type and have a fixed length
    let _a = [1, 2, 3, 4, 5];
    // are more useful when you know the number of elements will not need to change
    const MONTHS:  [&str; 12] = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
    // contain 5 elements that will all be set to the initially value
    let _b = [5; 5];
    // accessing array element
    println!("The first months of the year is {}", MONTHS[0]);
    // invalid array element access
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index entered was not a number");
    let element = MONTHS[index];
    println!("The value of the element at index {} is : {}", index, element);

    println!("-----------function-----------");
    another_fn();
    let sum = addition(5, 10);
    println!("{}", sum);

    println!("-----------control flow-----------");
    // if expression
    let num = 3;

    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let x = 0;
    if x != 0 {
        println!("x is not equal to 0");
    }

    // terminal 
    let condition = true;
    let number = if condition { 5 } else { 7 };
    println!("The value of number is : {}", number);

    // loop
    loop {
        println!("this is loop!")
        break;
    }

    // while loop

    // for loop
    let a = [1, 2, 3, 4, 5];
    for ele in a {
        println!("{}", ele);
    }

    for i in 1..4 {
        println!("{}", i);
    }
}

fn another_fn() {
    println!("This is another function!");
}

fn addition(x: u32, y: u32) -> u32 {
    x + y
}



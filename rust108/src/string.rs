// common collection string
// strings are implemented as a collection of bytes

use std::ops::Add;

#[allow(dead_code)]
fn main() {
    let _s1 = String::new();
    let _s2 = String::from("hello");

    let data = "initial contents";
    let s3 = &data.to_string();
    let s4 = &data.to_owned();
    println!("{:?}, {:?}", s3, s4);

    // updating string
    let mut s5 = String::from("foo");
    s5.push_str(" bar");
    println!("{s5}");

    let mut s6 = String::from("fooz");
    let s7 = "barz";
    s6.push_str(s7);
    println!("s7 still valid {}", s7);

    // concatenation with the + operation or the format! macro
    let s1 = String::from("hello ");
    let s2 = String::from("world");

    let s3 = s1.clone() + &s2;
    println!("{s3}");
    println!("s2 still valid {}", s2);


    let s4 = s1.add(&s2);
    println!("{s4}");

    // adding multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = s1.clone() + "-" + &s2 + "-" + &s3;
    println!("using + operation: {s4}");

    // for more complicate concatenation we use format! instead
    let s5 = format!("{}-{}-{}", s1, s2, s3);
    println!("using format!: {}", s5);

    // indexing into Strings
    // in rust Sring cannot be indexed by {interger}
    // how rust store strings in memory
    // internal representation
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for c in "नमस्ते".bytes() {
        println!("{}", c as char);
    }
}
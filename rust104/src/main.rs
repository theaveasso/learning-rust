// Understand ownership
mod slice_type;
pub use slice_type::*;

fn main() {
    // memory and allocation 
    // ownership and String
    // ways variabless and data interact: move : memory curruption 
    let s1 = String::from("Hello");
    let _s2 = s1;

    // ways variabless and data interact: clone (deep copy)
    let s3 = String::from("Hello");
    let _s4 = s3.clone();

    // stack-only data: copy
    // all int types, bool, float, char, tuple that contain type that also implement (i32, i32) copy
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // ownership and function
    let s = String::from("Hello");
    takes_os(s);
    // println!("{}", s) // borrow of moved value `s`
    
    let x = 5;
    makes_cp(x);
    println!("{}", x);

    // return values and scope
    // return values can also transfer ownership
    let _s1 = gives_os();
    let s2 = String::from("Jello");
    let _s3 = takes_and_gives_os(s2);

    // references and borrowing
    // it doesn't work if we try to borrow value we don't own
    let s = String::from("hello");
    let len = get_length(&s);
    println!("The length of {} is {}", s, len);

    // mutable references
    // you can have on;y one mutable reference to a particular piece of data at a time
    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(", world");
    println!("{}", r1);

    // slice type
    let s = String::from("hello world");
    let _a = &s[..5];
    let _b= &s[6..];

}

fn get_length(s: &String) -> usize {
    s.len()
}

fn takes_os(s: String) { // s comes into scope
    println!("{}", s);
}   // s goes out of scope and drop is called.

fn makes_cp(i: i32) {  // i comes into scope
    println!("{}", i)
}   // i goes out of scope, nothing specail hapen

fn gives_os() -> String { // will move its return value into the func that calls it
    let s = String::from("Hello");
    s
}

fn takes_and_gives_os(s: String) -> String {
    s
}


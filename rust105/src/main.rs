// Defining and instantiating Struct
mod example_of_using_struct;
pub use example_of_using_struct::*;

#[derive(Debug)]
struct User {
    activate: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Position(u32, f32, u32);

#[derive(Debug)]
struct Color(u8, u8, u8);

fn main() {
    let mut user1 = User {
        activate: true,
        username: String::from("theaveasso"),
        email: String::from("theaveasso@email.com"),
        sign_in_count: 2
    };

    user1.activate = false;
    print!("{:?}", user1);


    let user2 = build_user("taly".to_string(), "taly@email.com".to_string());
    print!("{:?}", user2);

    // create instance from other instance with struct update syntax
    let _user3 = User {
        activate: user1.activate,
        username: user2.username,
        email: user1.email,
        sign_in_count: 1,
    };

    // tuple structs without name fields to create different types
    let point_a = Position(3, 4.0, 8);
    let red = Color(255, 0, 0);

    println!("position of point a : {:?}", point_a);
    println!("color red value : {:?}", red);

}

fn build_user(username: String, email: String) -> User {
    User {
        activate: true,
        username, // using field shorthand
        email,
        sign_in_count: 1
    }
}
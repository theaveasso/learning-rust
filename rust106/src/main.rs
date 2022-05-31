// enum and pattern matching

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum _Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(u8, u8, u8)
}
impl _Message {
    fn _call(&self) {
        // func
    }
}

// <T> mean the some variant of the Option enum can hold one piece of data of "any type" and that each concrete ype that gets used in place of T
// enum  Option<T> {
//     None,
//     Some(T)
// }

// #[derive(Debug)]
// struct IpAddr {
//     kind : IpAddrKind,
//     address : String
// }

fn main() {
    // create instance of each of the two variants 
    let _ip_four = IpAddrKind::V4;
    let _ip_six = IpAddrKind::V6;

    // dbg!(&ip_four, &ip_six);

    /* 
    let home = IpAddr {
        kind : IpAddrKind::V4,
        address : String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind : IpAddrKind::V6,
        address : String::from("::1")
    };

    */

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    dbg!(&home, &loopback);
    'a' as ud);

    // enum could has a wide variety of types embedded in its variants
    let some_number = Some(5);
    let some_string = Some("as string");

    let absent_number: Option<i32> = None;
    println!("{:#?}, {:#?}, {:#?}", some_number, some_string, absent_number);



}


use std::fs::{File};
use std::io::{self, ErrorKind, Read};


#[allow(dead_code)]
pub fn main() {
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Prblem opening the file: {:?}", error);
        }
    });

    // matching on different erros
    /* 
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_e => {
                panic!("Problem opening the file: {:?}", other_e)
            }
        },
    };
    */
    // shortcut for panic or error: unwrap and expect
    let _f1 = File::open("hi.txt")
        .unwrap(); 
    // without hi.txt, will see an error message from panic!

    let _f2 = File::open("hola.txt")
        .expect("Failed to open hola.txt");

    // propagating errors
    let name = read_username_from_file();
    print!("{:?}", name);
    let last_char = last_char_of_line(&name.unwrap());
    println!("{:?}", last_char);



}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn last_char_of_line(txt: &str) -> Option<char> {
    txt.lines().next()?.chars().last()
}
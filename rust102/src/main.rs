// programming a gussing game
use std::io;
use rand::Rng;
use colored::*;
use std::cmp::Ordering;

fn main() {
    let secret_number: u8 = rand::thread_rng().gen_range(1..=101);
    
    println!("{}", "The Guessing Game!".green());
    loop {
        
        println!("{}", "Please Input the number".yellow());

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        // compare guess and secret num
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Go Higher!".red()),
            Ordering::Greater => println!("{}", "Go Smaller!".red()),
            Ordering::Equal => {
                println!("{}", "You win! 🥳".green());
                break;
            }
        }
    }
}

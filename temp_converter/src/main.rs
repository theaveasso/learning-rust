use std::io;

#[derive(Debug)]
enum Temperature {
    Celsius,
    Fahrenheit
}

fn main() {
    println!("Temperature converter!");
    
    let i_temperature = get_temperature();
    
    match get_temp_symbol() {
        Temperature::Celsius => 
            println!("{i_temperature} C is equal {:.2} F", i_temperature * 1.8 + 32.0),


        Temperature::Fahrenheit => 
            println!("{i_temperature} F is equal {:.2} C", (i_temperature + 32.0) / 1.8),
    };
}

fn get_temperature() -> f32 {
    println!("What is the temperature that you want to convert?");
    let mut i_temp = String::new();
    io::stdin()
        .read_line(&mut i_temp)
        .expect("Fail to read lines");
    
    let i_temp: f32 = i_temp.trim().parse()
        .expect("Please input a number next time!");

    
    return i_temp;
}

fn get_temp_symbol() -> Temperature {
    let mut i_symbol = String::new();
    println!("Is this a temperature in C or F ?");
    io::stdin()
        .read_line(&mut i_symbol)
        .expect("Fail to read lines");

    let symbol = match i_symbol.trim(){
        "c" => Temperature::Celsius,
        "f" => Temperature::Fahrenheit,
        _ => panic!()
    };
    return symbol;
    
    
    
}
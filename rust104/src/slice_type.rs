fn main() {
    let s = String::from("Hello Rustacean");
    
    let word = first_word(&s);


    println!("word : {}", word);

    let s2 = "Hello Rustacean";
    let word =  first_word(s2);

    println!("word : {}, {}", word, s2);
    
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}
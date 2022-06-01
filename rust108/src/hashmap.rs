// storing keys with associated values in hash maps
use std::collections::HashMap;

#[allow(unused)]
fn main() {
    // creating a new hm
    let mut scores = HashMap::new();
    scores.insert(String::from("Real Madrid"), 1);
    scores.insert(String::from("Liverpool"), 0);

    let teams = vec![String::from("White"), String::from("Red")];
    let result = vec![1, 0];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(result.into_iter()).collect();
    println!("{:?}", scores);

    // hashmap and ownershop
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Green");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // if we insert ref to values into the hash map, the values won't be moved into the hashmap,
    // the value that the references point to must be valid for at least as long as the hash map is valid

    // Accessing values in a hash map
    let team_name = String::from("White");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    // iterate key value pair
    for (k, v) in &scores {
        println!("{}  {}", k, v);
    }

    // updating a hash map
    // overiding a value
    scores.insert(String::from("White"), 14);

    // only inserting a value if the key has no value
    scores.entry(String::from("White")).or_insert(5);
    scores.entry(String::from("Blue")).or_insert(5);
    println!("{:?}", scores);

    // update a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
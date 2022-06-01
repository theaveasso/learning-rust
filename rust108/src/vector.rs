// vector

#[derive(Debug)]
enum SpreedsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

#[allow(dead_code)]
fn main() {
    // craeting a new vector
    let _v1: Vec<i32> = Vec::new();

    // store with initial value
    let _v2 = vec![1, 2, 3];

    // update vector
    let mut v3 = Vec::new();

    v3.push(10);
    v3.push(20);
    v3.push(30);
    v3.push(40);

    // dropping a vector, drop its elements
    {
        let _v4 = vec!['a', 'b', 'c'];
    } // v4 goew out of scope and freed here
    // println!("{}", v4)

    // reading elements of vectors
    // let _does_not_exist_1 = &v3[100];    // index out of bounds
    let does_exist_2 = v3.get(100); // return None

    println!("{:?}", does_exist_2);

    let e1 = &v3[0];

    println!("{:?}", e1);

    v3.push(50);

    // iterating over the values in a vector
    for i in &v3 {
        println!("{}", i);
    }
    
    // iterate over mutable ref to each element in a mut vector in order to 
    // make a changes to all the element
    for i in &mut v3 {
        *i += 1;
        println!("{}", i);
    }

    // using an enum to store multiple type
    let v5 = vec![
        SpreedsheetCell::Text(String::from("Jello!")),
        SpreedsheetCell::Int(32),
        SpreedsheetCell::Float(64.0)
    ];
    println!("{:?}", v5);


}
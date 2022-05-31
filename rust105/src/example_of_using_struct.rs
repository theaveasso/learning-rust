
#[derive(Debug)]
struct Rectangle {
    width : u32, 
    height : u32
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        &self.width * &self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
    
}

fn main() {
    let w = 30;
    let h = 50;

    let rect_a = Rectangle { width: 80, height: 50 };

    let rect_b = Rectangle { width: 30, height: 40 };
    dbg!(&rect_b);
    println!("Area of rectagle B is {}", rect_b.get_area());

    println!("Can rect a hold rect b {}", rect_a.can_hold(&rect_b));

    let square_a = Rectangle::square(50);
    dbg!(&square_a);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
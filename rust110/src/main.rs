use rust110::{Tweet, Summary, NewArticle};

// Generic types
/*
    - Reduce code duplication
    - Struct Definition


*/
mod generic;
mod lib;

#[allow(unused, dead_code)]
#[derive(Debug)]
struct Point<T: Copy + Clone, U> {
    x: T,
    y: U,
}

struct ImportantExcerpt<'a> {
    part: &'a str
}

#[allow(unused, dead_code)]
impl<T: Copy, U> Point<T, U> {
    fn x(self) -> T {
        self.x
    }

    fn y(self) -> U {
        self.y
    }
    
    fn mixup<V: Copy, R> (self, other: Point<V, R>) -> Point<T, R> {
        Point { x: self.x, y: other.y}
    }
}
#[allow(unused, dead_code)]
impl Point<i32, f32> {
    fn add(self) -> f32 {
        self.y + 2.0
    }
    
}
#[allow(unused, dead_code)]
struct NumberWrapper<T: std::ops::Add<Output = T> + Copy> {
    data: T
}

#[allow(unused, dead_code)]
fn main() {
    // removing duplication by extracint function
    let list1 = Vec::from([33, 3432, 34, 9830, 10495]);
    let n = get_largest(&list1);
    println!("{}", n);

    let list2 = Vec::from(['h', 'e', 'l', 'o', 'w', 'r', 'd']);
    let c = get_largest(&list2);
    println!("{}", c);

    let p1 = Point { x: 5, y: 10.0};
    let p2 = Point { x: -343, y: 'c'};
    dbg!(p1.mixup(p2));

    let sum = double_it(n);
    let sum = double_it(15.5_f32);

    let num1 = NumberWrapper { data: 1_u32};
    let num2 = NumberWrapper{ data: 5.5_f32};

    // traits
    let my_tweet = Tweet {
        username: String::from("theaveasso"),
        content: String::from("I'm learning rust"),
        retweet: false,
        reply: false
    };
    println!("1 new tweet: {}", my_tweet.summurize());

    // default traits
    let my_new_article = NewArticle {
        headline: String::from("Penguin win the Stanley Cup championship"),
        location: String::from("Pittsburgh, PA, USA"),
        content: String::from("
            The Pittburgh Penguins once aganin....
        "),
        author: String::from("Iceburgh")
    };

    println!("1 new article: {}", my_new_article.summurize());
    
    notify1(&my_new_article);

}

fn get_largest<T: PartialOrd + Copy> (list: &Vec<T>) -> T { // trait
    let mut max = list[0];

    for &n in list {
        if n > max {
            max = n;
        }
    }
    return max;
}

fn double_it<T: std::ops::Add<Output = T> + Copy>(numb: T) -> T {
    numb + numb
}

fn notify1(item: &impl Summary) {
    println!("Breaking news: {}", item.summurize())
}

fn notify<T: Summary> (item: &T) {
    println!("Breaking news: {}", item.summurize())
}

pub trait Summary {
    fn summurize_author(&self) -> String;
    fn summurize(&self) -> String {
        format!("(Read more...) {}", self.summurize_author())
    }
}

pub struct NewArticle {
    pub location: String,
    pub headline: String,
    pub content: String,
    pub author: String,
}
impl Summary for NewArticle {
    fn summurize_author(&self) -> String {
        format!("@{}", self.author)
    }
    
}
// impl Summary for NewArticle {
//     fn summurize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summurize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summurize(&self) -> String {
        format!("{}: {}", self.summurize_author(), self.content)
    }
    
}

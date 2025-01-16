use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &(impl Summary + Display)) {
    println!("Breaking news! {} {} ", item, item.summarize());
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}|{}|{}|{}|",
            self.headline, self.location, self.author, self.content
        )
    }
}

fn main() {
    let news = NewsArticle {
        headline: "headline".to_string(),
        location: "location unknown".to_string(),
        author: "author".to_string(),
        content: "'content".to_string(),
    };

    notify(&news);

    println!("summarize : {}", news.summarize());
}

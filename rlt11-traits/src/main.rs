fn main() {
    t1();
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {}", &self.headline, &self.author)
//     }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", &self.content, &self.username)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news 2! {}", item.summarize())
}

pub fn notify34(item: &impl Summary, item2:&impl Summary) {
    println!("Breaking news 3 & 4! {} {}", item.summarize(), item2.summarize());
}

pub fn notify56<T: Summary>(item: &T, item2: &T) {
    println!("Breaking news 5 & 6! {} {}", item.summarize(), item2.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("cj"),
        content:  String::from("bobby"),
        reply: false,
        retweet: false,
    }
}

fn t1() {
    let tweet = Tweet {
        username: String::from("carlton"),
        content:  String::from("hi there"),
        reply: false,
        retweet: false,
    };

    println!("Tweet Summary; {}", tweet.summarize());
    
    let article = NewsArticle {
        author: String::from("carlton"),
        headline:  String::from("who is bob"),
        content:  String::from("he is your uncle"),
    };

    println!("Article Summary; {}", article.summarize());
    notify(&article);
    notify2(&tweet);
    notify34(&tweet, &article);
    notify56(&tweet, &tweet);
    let tweet2 = returns_summarizable();
    println!("{}", tweet2.summarize());
}

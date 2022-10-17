//use ch10_2::{Summary, Tweet, NewsArticle, };
//use crate::{Summary, Tweet};

use ch10_2b::{NewsArticle, Summary, Tweet};


fn main() {
    println!("ch10-2 starts!!");

    let tweet = Tweet {
        username: "house_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: "Penguins win the Stanley Cup Championship!".to_string(),
        location: "Pittsburgh, PA, USA".to_string(),
        author: "Iceburgh".to_string(),
        content: "The Pittsburgh Penguins once again are the best hockey team in the NHL.".to_string()
    };

    println!("New article available! {}", article.summarize());
}

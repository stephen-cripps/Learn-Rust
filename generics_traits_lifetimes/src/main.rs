mod aggregator;
use aggregator::{NewsArticle, Summary, Tweet};

use crate::aggregator::notify;

fn main() {
    Generics();
    Traits();
}

fn Generics() {
    let number_list = vec![34, 50, 25, 67, 98];
    let largest = Largest(&number_list);
    println!("Largest number is {largest}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest = Largest(&char_list);
    println!("Largest char is {largest}")
}

fn Largest<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];

    // Won't compile because we need a trait
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn Traits() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("I Liek Horz"),
        reply: false,
        retweet: false,
    };

    notify(&tweet);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    notify(&article);
}

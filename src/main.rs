use std::fmt::Display;
fn main() {
    let number_list = vec![1, 3, 45, 7, 8, 4, 63, 6];
    let char = vec!['t', 'f', 'g', 'f', 'a', 'v', 'x'];
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         larget = number;
    //     }
    // }
    // println!("The largest number is {}", largest);
    // assert_eq!(larget, 63);
    let t = largest_i32(&number_list);
    let char_1 = largest_char(&char);
    println!("The largest number is {}", t);
    println!("The largest char is {}", char_1);
    let t = Point { x: 4.0, y: 4.5 };
    let y = Point { x: 'r', y: 'y' };
    let u = PointF { x: 'f', y: 'r' };
    println!("y.x ={}", t.distance_from_origin());
    let p1 = PointF { x: 3, y: 10.4 };
    let p2 = p1.mixup(u);
    //x is the T, y is the V
    println!("p2.x-{}, p2.y -{}", p2.x, p2.y);

    let tweet = tweet {
        username: String::from("Ndeta innocent"),
        content: String::from("Thanks for contributing"),
        reply: false,
        retweet: false,
    };
    println!(" A new tweet {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Is reccesion commin?"),
        location: String::from("Katusepapi 13"),
        author: String::from("Ndeta Innocent"),
        content: String::from("We are facing an economic decline..."),
    };
    println!("{}", article.summarize());
    //Try using the new notify fn
    notify(&article);
    //Using generics to fix our types and enable our function compare ints and chars
    let num_1 = largest_generic(&char);
    println!("The largest char is {}", num_1);
}
//Generic type struct but both elements must be of same generic types
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
//Generic type struct but both elements can be of different generic types
struct PointF<T, U> {
    x: T,
    y: U,
}
impl<T, U> PointF<T, U> {
    fn mixup<V, W>(self, other: PointF<V, W>) -> PointF<T, W> {
        PointF {
            x: self.x,
            y: other.y,
        }
    }
}
fn largest(vec: &[i32]) -> &i32 {
    let mut large = &vec[0];
    for num in vec {
        if num > large {
            large = num
        }
    }
    large
}
//Generic types
fn largest_i32(vec: &[i32]) -> &i32 {
    let mut large = &vec[0];
    for num in vec {
        if num > large {
            large = num
        }
    }
    large
}
fn largest_char(vec: &[char]) -> &char {
    let mut large = &vec[0];
    for num in vec {
        if num > large {
            large = num
        }
    }
    large
}
fn largest_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

//Traits
//Trait definition without default implementation
pub trait summary {
    fn summarize(&self) -> String;
}
//Trait definition with implementation
pub trait summary_def {
    fn summarize(&self) -> String {
        String::from("(Reade more...)")
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
//This will overwrite the default implimentation of summarize
impl summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "Headline for today. :{}, by:__ {} Location:__({})",
            self.headline, self.author, self.location
        )
    }
}
//Default implementation
//impl summary_def for NewsArticle {}
pub struct tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl summary for tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
//Traits as parameters

pub fn notify(item: &impl summary) {
    println!("Breaking news! {}", item.summarize())
}

//Trait bound synthax
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("THe largest number is x _{}", self.x);
        } else {
            println!("The largest number is y __ {}", self.y);
        }
    }
}

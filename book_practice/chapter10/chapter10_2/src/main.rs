mod aggregator {
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

    /*
    为 NewsArticle 和 Tweet 类型实现了 Summary trait，
    用其来定义了一个函数 notify 来调用其参数 item 上的 summarize 方法，
    该参数是实现了 Summary trait 的某种类型。为此可以使用 impl Trait 语法，像这样：
    */
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    /*
    impl Trait 语法适用于直观的例子，它实际上是一种较长形式语法的语法糖。我们称为 trait bound，它看起来像：

    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    通过 + 指定多个 trait bound
    如果 notify 需要显示 item 的格式化形式，同时也要使用 summarize 方法，
    那么 item 就需要同时实现两个不同的 trait：Display 和 Summary。这可以通过 + 语法实现：

        pub fn notify(item: &(impl Summary + Display)) {

    + 语法也适用于泛型的 trait bound：
        pub fn notify<T: Summary + Display>(item: &T) {

    通过指定这两个 trait bound，notify 的函数体可以调用 summarize 并使用 {} 来格式化 item。
    */
}

use aggregator::{NewsArticle, Summary, Tweet};

use std::fmt::Display;

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
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {}

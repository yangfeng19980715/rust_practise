#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // fn area(&self) -> u32 {
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

    // 我们可以选择将方法的名称与结构中的一个字段相同
    /*
    在 main 中，当我们在 rect1.width 后面加上括号时。Rust 知道我们指的是方法 width。当我们不使用圆括号时，Rust 知道我们指的是字段 width。
    通常，但并不总是如此，
    与字段同名的方法将被定义为只返回字段中的值，而不做其他事情。这样的方法被称为 getters，Rust 并不像其他一些语言那样为结构字段自动实现它们。
    Getters 很有用，因为你可以把字段变成私有的，但方法是公共的，这样就可以把对字段的只读访问作为该类型公共 API 的一部分。
    */
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.width > other.width && self.height > other.height {
            return true;
        }

        return false;
    }

    /*
    所有在 impl 块中定义的函数被称为 关联函数（associated functions），因为它们与 impl 后面命名的类型相关。
    我们可以定义不以 self 为第一参数的关联函数（因此不是方法），因为它们并不作用于一个结构体的实例。

    不是方法的关联函数经常被用作返回一个结构体新实例的构造函数。这些函数的名称通常为 new ，但 new 并不是一个关键字。
    */
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

/* 每个结构体都允许拥有多个 impl 块。 */
impl Rectangle {
    fn display_hello(self: &Self) {
        println!("hello");
    }
}

fn main() {
    let rect0 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels. width > 0 : {}",
        rect0.area(),
        rect0.width()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);

    println!("{:?}", sq);
    sq.display_hello();
}

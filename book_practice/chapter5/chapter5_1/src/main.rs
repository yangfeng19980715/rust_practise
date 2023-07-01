struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: false,
        username: String::from("Bob"),
        email: String::from("111@gmail.com"),
        sign_in_count: 32,
    };

    println!("{}", user1.active);

    println!("Hello, world!");

    let user2 = build_user("hello".to_string(), "world".to_string());

    println!("{}", user2.email);

    let user3 = build_user_2(String::from("yes"), String::from("no"));
    println!("{}", user3.email);

    let user4 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user5 = User {
        email: String::from("double"),
        ..user4 // 这里是移动，所以后面 user4 的email之外的属性就没用了
    };

    // println!("{}", user4.username); // error
    println!("{}", user4.email); // ok

    // 元组结构体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0);

    // 类单元结构体
    // 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    /*
    struct User_2_t {
        active: bool,
        username: &str,
        email: &str,
        sign_in_count: u64,
    };

    // &str 字符串 slice 类型
    // error , 生命周期不对
    // 可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上 生命周期（lifetimes）
    let user1 = User_2_t {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
    */
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

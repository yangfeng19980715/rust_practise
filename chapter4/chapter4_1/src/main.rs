fn main() {
    /*
    let s = "hello";
    println!("s is {s}");

    let mut s = String::from("hello"); // 基于字符串字面值来创建String
    s.push_str(", world");
    println!("s is {s}");

    // Rust 采取了一个不同的策略：内存在拥有它的变量离开作用域后就被自动释放。


    let a = 5;
    let b = a;

    println!("a = {a}, b = {b}");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");


    let s = String::from("hello");
    takes_ownershipe(s);

    //  println!("after function, s = {s}");

    let x = 5;
    makes_copy(x);
    println!("after function, x = {x}");
    */

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    let (s4, len) = calculate_length(s3);

    println!("length of {s4} is {len}");

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}

fn gives_ownership() -> String {
    // gives_ownership 会将返回值移动给调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域。
    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}

fn takes_ownershipe(some_string: String) {
    println!("{}", some_string);
} // 调用 drop 方法

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}



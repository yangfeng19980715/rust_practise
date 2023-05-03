fn main() {

    let s = String::from("hello, world");


    let fw = first_word(&s);

    println!("fw : {fw}");

    let hello = &s[0..5];
    let hello_2 = &s[..5];
    let world = &s[6..12];
    let world_2 = &s[6..];

    let hello_world = &s[..];


    println!("hello:{hello}, world:{world}");
    println!("hello_2: {hello_2}, world_2: {world_2}");
    println!("hello_world: {hello_world}");

    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），部分或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，部分或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);
    println!("{my_string_literal}, {word}");

    let a = [9, 8, 6, 7, 4];

    let slice = &a[1..3];

    assert_eq!(slice, [8, 6]);

    for val in slice {
        println!("{val} ");
    }

}

/*
    这里有一个编程小习题：编写一个函数，该函数接收一个用空格分隔单词的字符串，并返回在该字符串中找到的第一个单词。如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。
*/

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}



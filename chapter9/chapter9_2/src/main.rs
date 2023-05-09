use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

/*
    大部分错误并没有严重到需要程序完全停止执行。
    有时，一个函数会因为一个容易理解并做出反应的原因失败。
    例如，如果因为打开一个并不存在的文件而失败，此时我们可能想要创建这个文件，而不是终止进程。
*/

/*
File::open 的返回值是 Result<T, E>。泛型参数 T 会被 File::open 的实现放入成功返回值的类型 std::fs::File，这是一个文件句柄。错误返回值使用的 E 的类型是 std::io::Error。这些返回类型意味着 File::open 调用可能成功并返回一个可以读写的文件句柄。这个函数调用也可能会失败：例如，也许文件不存在，或者可能没有权限访问这个文件。File::open 函数需要一个方法在告诉我们成功与否的同时返回文件句柄或者错误信息。这些信息正好是 Result 枚举所代表的。

当 File::open 成功时，greeting_file_result 变量将会是一个包含文件句柄的 Ok 实例。当失败时，greeting_file_result 变量将会是一个包含了更多关于发生了何种错误的信息的 Err 实例。
*/
fn test_func() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn test_func2() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn test_func3() {
    /*
    match 能够胜任它的工作，不过它可能有点冗长并且不总是能很好的表明其意图。
    Result<T, E> 类型定义了很多辅助方法来处理各种情况。
    其中之一叫做 unwrap，它的实现就类似于示例 9-4 中的 match 语句。
    如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。如果 Result 是成员 Err，unwrap 会为我们调用 panic!
    */
    // let greeting_file = File::open("hello.txt").unwrap();

    /*
    还有另一个类似于 unwrap 的方法它还允许我们选择 panic! 的错误信息：expect。
    使用 expect 而不是 unwrap 并提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源。
    */
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    /*
    expect 与 unwrap 的使用方式一样：返回文件句柄或调用 panic! 宏。
    expect 在调用 panic! 时使用的错误信息将是我们传递给 expect 的参数，而不像 unwrap 那样使用默认的 panic! 信息。
    */
}

/*
当编写一个其实先会调用一些可能会失败的操作的函数时，除了在这个函数中处理错误外，还可以选择让调用者知道这个错误并决定该如何处理。
这被称为 传播（propagating）错误，这样能更好的控制代码调用，因为比起你代码所拥有的上下文，调用者可能拥有更多信息或逻辑来决定应该如何处理错误。
*/
fn test_func4() -> Result<String, io::Error> {
    // fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        // 在 Err 的情况下，我们没有调用 panic!，而是使用 return 关键字提前结束整个函数，
        // 并将来自 File::open 的错误值（现在在模式变量 e 中）作为函数的错误值传回给调用者。
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        // 如果read_to_string 失败了，则像之前处理 File::open 的返回值的 match 那样返回错误值。
        // 不过并不需要显式的调用 return，因为这是函数的最后一个表达式。
        Err(e) => Err(e),
    }

    /* 这种传播错误的模式在 Rust 是如此的常见，以至于 Rust 提供了 ? 问号运算符来使其更易于处理。 */
}

// 它实现了与示例 test_func4() 中的代码相同的功能，不过这个实现使用了 ? 运算符
fn test_func5() -> Result<String, io::Error> {
    // fn read_username_from_file() -> Result<String, io::Error> {

    /*
    Result 值之后的 ? 被定义为与示例 9-6 中定义的处理 Result 值的 match 表达式有着完全相同的工作方式。
    如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
    如果值是 Err，Err 中的值将作为整个函数的返回值，就好像使用了 return 关键字一样，这样错误值就被传播给了调用者。
    */
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)

    /*
    File::open 调用结尾的 ? 会将 Ok 中的值返回给变量 username_file。
    如果发生了错误，? 运算符会使整个函数提前返回并将任何 Err 值返回给调用代码。同理也适用于 read_to_string 调用结尾的 ?。

    ? 运算符消除了大量样板代码并使得函数的实现更简单。我们甚至可以在 ? 之后直接使用链式方法调用来进一步缩短代码。
    */
}

fn test_func6() -> Result<String, io::Error> {
    // fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

/*
    将文件读取到一个字符串是相当常见的操作，所以 Rust 提供了名为 fs::read_to_string 的函数，
    它会打开文件、新建一个 String、读取文件的内容，并将内容放入 String，接着返回它。
*/
fn test_func7() -> Result<String, io::Error> {
    // fn read_username_from_file() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}

/*
? 运算符只能被用于返回值与 ? 作用的值相兼容的函数。因为 ? 运算符被定义为从函数中提早返回一个值。
*/

/*
? 也可用于 Option<T> 值。如同对 Result 使用 ? 一样，只能在返回 Option 的函数中对 Option 使用 ?。
在 Option<T> 上调用 ? 运算符的行为与 Result<T, E> 类似：如果值是 None，此时 None 会从函数中提前返回。
如果值是 Some，Some 中的值作为表达式的返回值同时函数继续。
*/

/*
这个函数返回 Option<char> 因为它可能会在这个位置找到一个字符，也可能没有字符。
这段代码获取 text 字符串 slice 作为参数并调用其 lines 方法，这会返回一个字符串中每一行的迭代器。
因为函数希望检查第一行，所以调用了迭代器 next 来获取迭代器中第一个值。
如果 text 是空字符串，next 调用会返回 None，此时我们可以使用 ? 来停止并从 last_char_of_first_line 返回 None。
如果 text 不是空字符串，next 会返回一个包含 text 中第一行的字符串 slice 的 Some 值。

? 会提取这个字符串 slice，然后可以在字符串 slice 上调用 chars 来获取字符的迭代器。
我们感兴趣的是第一行的最后一个字符，所以可以调用 last 来返回迭代器的最后一项。
这是一个 Option，因为有可能第一行是一个空字符串，例如 text 以一个空行开头而后面的行有文本，像是 "\nhi"。
不过，如果第一行有最后一个字符，它会返回在一个 Some 成员中。
? 运算符作用于其中给了我们一个简洁的表达这种逻辑的方式。
如果我们不能在 Option 上使用 ? 运算符，则不得不使用更多的方法调用或者 match 表达式来实现这些逻辑。
*/
fn test_func8(text: &str) -> Option<char> {
    // fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

/*
注意你可以在返回 Result 的函数中对 Result 使用 ? 运算符，可以在返回 Option 的函数中对 Option 使用 ? 运算符，但是不可以混合搭配。
? 运算符不会自动将 Result 转化为 Option，反之亦然；在这些情况下，可以使用类似 Result 的 ok 方法或者 Option 的 ok_or 方法来显式转换。
*/

/*
main 函数的返回类型是 () 而不是 Result。
*/

fn main() {
    // test_func();
    // test_func2();

    // test_func4();
    // test_func5();

    // test_func6();
    test_func8("hello.txt");
}

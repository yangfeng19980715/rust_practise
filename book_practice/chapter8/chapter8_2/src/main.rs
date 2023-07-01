/*
    在集合章节中讨论字符串的原因是，字符串就是作为字节的集合外加一些方法实现的，当这些字节被解释为文本时，这些方法提供了实用的功能。
    在这一部分，我们会讲到 String 中那些任何集合类型都有的操作，比如创建、更新和读取。
    也会讨论 String 与其他集合不一样的地方，例如索引 String 是很复杂的，由于人和计算机理解 String 数据方式的不同。
*/

/*
    Rust 的核心语言中只有一种字符串类型：字符串 slice str，它通常以被借用的形式出现，&str。
    第四章讲到了 字符串 slices：它们是一些对储存在别处的 UTF-8 编码字符串数据的引用。
    举例来说，由于字符串字面值被储存在程序的二进制输出中，因此字符串字面值也是字符串 slices。
*/

fn test_func1() {
    /* 很多 Vec 可用的操作在 String 中同样可用，事实上 String 被实现为一个带有一些额外保证、限制和功能的字节 vector 的封装。 */
    let mut s = String::new();

    /* 通常字符串会有初始数据，因为我们希望一开始就有这个字符串。为此，可以使用 to_string 方法，它能用于任何实现了 Display trait 的类型，比如字符串字面值。*/
    let data = "initial contents";
    let s = data.to_string();

    // 该方法也可直接用于字符串字面值：
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    /* 记住字符串是 UTF-8 编码的，所以可以包含任何可以正确编码的数据，如示例 8-14 所示。 */

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    /*
        String 的大小可以增加，其内容也可以改变，就像可以放入更多数据来改变 Vec 的内容一样。
        另外，可以方便的使用 + 运算符或 format! 宏来拼接 String 值。
    */

    let mut s = String::from("foo");
    println!("s.len: {}", s.len());
    s.push_str("bar");
    println!("s.len: {}", s.len());

    /* push_str 方法采用字符串 slice，因为我们并不需要获取参数的所有权。 */
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');

    /* 通常你会希望将两个已知的字符串合并在一起。一种办法是像这样使用 + 运算符。 */
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    println!("s3:{}", s3);

    /*
        s1 在相加后不再有效的原因，和使用 s2 的引用的原因，与使用 + 运算符时调用的函数签名有关。+ 运算符使用了 add 函数，这个函数签名看起来像这样：
        fn add(self, s: &str) -> String {
    */

    /*
        之所以能够在 add 调用中使用 &s2 是因为 &String 可以被 强转（coerced）成 &str。
        当add函数被调用时，Rust 使用了一个被称为 Deref 强制转换（deref coercion）的技术，你可以将其理解为它把 &s2 变成了 &s2[..]。
        因为 add 没有获取参数的所有权，所以 s2 在这个操作后仍然是有效的 String。

        其次，可以发现签名中 add 获取了 self 的所有权，因为 self 没有 使用 &。
        这意味着示例中的 s1 的所有权将被移动到 add 调用中，之后就不再有效。
        所以虽然 let s3 = s1 + &s2; 看起来就像它会复制两个字符串并创建一个新的字符串，而实际上这个语句会获取 s1 的所有权，附加上从 s2 中拷贝的内容，并返回结果的所有权。
        换句话说，它看起来好像生成了很多拷贝，不过实际上并没有：这个实现比拷贝要更高效。
    */

    /*
        如果想要级联多个字符串，+ 的行为就显得笨重了。
        对于更为复杂的字符串链接，可以使用 format! 宏。
        宏 format! 生成的代码使用引用所以不会获取任何参数的所有权。
    */
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("s:{}", s);

    /*
        在很多语言中，通过索引来引用字符串中的单独字符是有效且常见的操作。
        然而在 Rust 中，如果你尝试使用索引语法访问 String 的一部分，会出现一个错误。

        错误和提示说明了全部问题：Rust 的字符串不支持索引。
    */
    // let s1 = String::from("hello");
    // let h = s1[0];

    let hello = "Здравствуйте";
    println!("hello.len:{}", hello.len());

    /*
        这引起了关于 UTF-8 的另外一个问题：
            从 Rust 的角度来讲，事实上有三种相关方式可以理解字符串：字节、标量值和字形簇（最接近人们眼中 字母 的概念）。

        Rust 提供了多种不同的方式来解释计算机储存的原始字符串数据，这样程序就可以选择它需要的表现方式，而无所谓是何种人类语言。

        最后一个 Rust 不允许使用索引获取 String 字符的原因是，索引操作预期总是需要常数时间（O(1)）。
        但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符。

    */

    /*
        索引字符串通常是一个坏点子，因为字符串索引应该返回的类型是不明确的：字节值、字符、字形簇或者字符串 slice。
        因此，如果你真的希望使用索引创建字符串 slice 时，Rust 会要求你更明确一些。
        为了更明确索引并表明你需要一个字符串 slice，相比使用 [] 和单个值的索引，可以使用 [] 和一个 range 来创建含特定字节的字符串 slice。
    */
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("s:{}", s);
    /*
        如果获取 &hello[0..1] 会发生什么呢？
        答案是：Rust 在运行时会 panic，就跟访问 vector 中的无效索引时一样。
    */

    // let s = &hello[0..1];
    // println!("s:{}", s);

    /*
        操作字符串每一部分的最好的方法是明确表示需要字符还是字节。
        对于单独的 Unicode 标量值使用 chars 方法
        。对 “Зд” 调用 chars 方法会将其分开并返回两个 char 类型的值，接着就可以遍历其结果来访问每一个元素了：
    */
    for c in "Зд".chars() {
        println!("{c}");
    }

    /* 另外 bytes 方法返回每一个原始字节 */
    for b in "Зд".bytes() {
        println!("{b}");
    }

    /* 有效的 Unicode 标量值可能会由不止一个字节组成。 */

    /*
        contains 来搜索一个字符串，和 replace 将字符串的一部分替换为另一个字符串。
        称作 String 的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。
        这两个类型在 Rust 标准库中都被广泛使用，String 和字符串 slices 都是 UTF-8 编码的。
    */
}

fn main() {
    test_func1();
}

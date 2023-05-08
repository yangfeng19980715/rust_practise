fn main() {
    let mut v1: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    v1.push(10);

    /*
        有两种方法引用 vector 中储存的值：通过索引或使用 get 方法。
        在接下来的示例中，为了更加清楚的说明，我们已经标注了这些函数返回的值的类型。
    */

    // test_func1();
    // test_func2();
    // test_func3();
    test_func4();
}

fn test_func4() {
    /*
        vector 只能储存相同类型的值。
        这是很不方便的；绝对会有需要储存一系列不同类型的值的用例。
        幸运的是，枚举的成员都被定义为相同的枚举类型，所以当需要在 vector 中储存不同类型值时，我们可以定义并使用一个枚举！
    */
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    /*
    Rust 在编译时就必须准确的知道 vector 中类型的原因在于它需要知道储存每个元素到底需要多少内存。
    第二个好处是可以准确的知道这个 vector 中允许什么类型。
    如果 Rust 允许 vector 存放任意类型，那么当对 vector 元素执行操作时一个或多个类型的值就有可能会造成错误。
    使用枚举外加 match 意味着 Rust 能在编译时就保证总是会处理所有可能的情况。

    如果在编写程序时不能确切无遗地知道运行时会储存进 vector 的所有类型，枚举技术就行不通了。
    相反，你可以使用 trait 对象。
    */

    /*
    当 vector 被丢弃时，所有其内容也会被丢弃，这意味着这里它包含的整数将被清理。借用检查器确保了任何 vector 中内容的引用仅在 vector 本身有效时才可用。
    */
        {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}

fn test_func3() {
    let v = vec![1, 2, 3, 4, 5];

    /* 使用 for 循环来获取 i32 值的 vector 中的每一个元素的不可变引用并将其打印 */
    for val in &v {
        print!("{}\t", val);
    }

    println!("");

    /*
        我们也可以遍历可变 vector 的每一个元素的可变引用以便能改变他们。
        为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值。
    */
    let mut v1 = vec![3, 4, 5, 6, 7];
    for val in &mut v1 {
        *val += 50;
    }

    for val in &v1 {
        println!("{}", val);
    }

    /*
        因为借用检查器的规则，无论可变还是不可变地遍历一个 vector 都是安全。
        for 循环中获取的 vector 引用阻止了同时对 vector 整体的修改。
    */
}

fn test_func1() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    /*
        使用 & 和 [] 会得到一个索引位置元素的引用。当使用索引作为参数调用 get 方法时，会得到一个可以用于 match 的 Option<&T>。
        Rust 提供了两种引用元素的方法的原因是当尝试使用现有元素范围之外的索引值时可以选择让程序如何运行。
    */
}

fn test_func2() {
    let mut v = vec![1, 2, 3, 4, 5];

    let elem = &v[4];
    println!("{}th element is {}", 4, elem);
    v.push(100);
    // let does_not_exist = v.get(100);
    /*
        当运行这段代码，你会发现对于第一个 [] 方法，当引用一个不存在的元素时 Rust 会造成 panic。
            这个方法更适合当程序认为尝试访问超过 vector 结尾的元素是一个严重错误的情况，这时应该使程序崩溃。
        当 get 方法被传递了一个数组外的索引时，它不会 panic 而是返回 None。
            当偶尔出现超过 vector 范围的访问属于正常情况的时候可以考虑使用它。接着你的代码可以有处理 Some(&element) 或 None 的逻辑。
    */

    /*

        为什么第一个元素的引用会关心 vector 结尾的变化？
        不能这么做的原因是由于 vector 的工作方式：在 vector 的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。
        这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。
    */
}

fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    /*
        如果值是 Some，我们希望打印出 Some 成员中的值，这个值被绑定到模式中的 max 变量里。对于 None 值我们不希望做任何操作。
        为了满足 match 表达式（穷尽性）的要求，必须在处理完这唯一的成员后加上 _ => ()，这样也要增加很多烦人的样板代码。
        不过我们可以使用 if let 这种更短的方式编写。
    */

    /*
        if let 语法获取通过等号分隔的一个模式和一个表达式。
        它的工作方式与 match 相同，这里的表达式对应 match 而模式则对应第一个分支。
        在这个例子中，模式是 Some(max)，max 绑定为 Some 中的值。
        接着可以在 if let 代码块中使用 max 了，就跟在对应的 match 分支中一样。模式不匹配时 if let 块中的代码不会执行。
    */

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    /*
        使用 if let 意味着编写更少代码，更少的缩进和更少的样板代码。然而，这样会失去 match 强制要求的穷尽性检查。match 和 if let 之间的选择依赖特定的环境以及增加简洁度和失去穷尽性检查的权衡取舍。
        换句话说，可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。
        可以在 if let 中包含一个 else。else 块中的代码与 match 表达式中的 _ 分支块中的代码相同，这样的 match 表达式就等同于 if let 和 else。
    */
}

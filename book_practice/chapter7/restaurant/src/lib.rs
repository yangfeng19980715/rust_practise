/*
    我们定义一个模块，是以 mod 关键字为起始，然后指定模块的名字（本例中叫做 front_of_house），并且用花括号包围模块的主体。
    在模块内，我们还可以定义其他的模块，就像本例中的 hosting 和 serving 模块。
    模块还可以保存一些定义的其他项，比如结构体、枚举、常量、特性、或者函数。
*/

/*
    src/main.rs 和 src/lib.rs 叫做 crate 根。
    之所以这样叫它们是因为这两个文件的内容都分别在 crate 模块结构的根组成了一个名为 crate 的模块，
        该结构被称为 模块树（module tree）。
*/

/*
    路径有两种形式：
        绝对路径（absolute path）是以 crate 根（root）开头的全路径；对于外部 crate 的代码，是以 crate 名开头的绝对路径，对于对于当前 crate 的代码，则以字面值 crate 开头。
        相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。
*/

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

/*  我们更倾向于使用绝对路径，因为把代码定义和项调用各自独立地移动是更常见的。*/
/*
    这表明使模块公有并不使其内容也是公有的。
    模块上的 pub 关键字只允许其父模块引用它，而不允许访问内部代码。
    因为模块是一个容器，只是将模块变为公有能做的其实并不太多；同时需要更深入地选择将一个或多个项变为公有。
    私有性规则不但应用于模块，还应用于结构体、枚举、函数和方法。
*/

/*
    我们还可以使用 super 而不是当前模块或者 crate 根来开头来构建从父模块开始的相对路径。
    这么做类似于文件系统中以 .. 开头的语法。使用 super 允许我们引用已知的父模块中的项，
        当模块与父模块关联的很紧密的时候，如果某天可能需要父模块要移动到模块树的其它位置，这使得重新组织模块树变得更容易。
*/

pub fn eat_at_restaurant_2() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house_2 {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

/*
    我们还可以使用 pub 来设计公有的结构体和枚举，不过关于在结构体和枚举上使用 pub 还有一些额外的细节需要注意。
    如果我们在一个结构体定义的前面使用了 pub ，这个结构体会变成公有的，但是这个结构体的字段仍然是私有的。
    我们可以根据情况决定每个字段是否公有。
*/

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");
}

/* 与之相反，如果我们将枚举设为公有，则它的所有成员都将变为公有。 */
mod back_of_house_3 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_3() {
    let order1 = back_of_house_3::Appetizer::Soup;
    let order2 = back_of_house_3::Appetizer::Salad;
}

/* 还有一种使用 pub 的场景我们还没有涉及到，那就是我们最后要讲的模块功能：use 关键字。 */

use crate::front_of_house::hosting;

pub fn eat_at_restaurant_4() {
    hosting::add_to_waitlist();
}

/*
    注意 use 只能创建 use 所在的特定作用域内的短路径。
*/

/*
    将 eat_at_restaurant 函数移动到了一个叫 customer 的子模块，这又是一个不同于 use 语句的作用域，所以函数体不能编译。
    注意这里还有一个警告说 use 在其作用域内不再被使用！
    为了修复这个问题，可以将 use 移动到 customer 模块内，或者在子模块 customer 内通过 super::hosting 引用父模块中的这个短路径。
*/
mod customer {
    pub fn eat_at_restaurant() {
        // super::hosting::add_to_waitlist(); // ok
        // hosting::add_to_waitlist(); // error
    }
}

/*
要想使用 use 将函数的父模块引入作用域，我们必须在调用函数时指定父模块，这样可以清晰地表明函数不是在本地定义的，同时使完整路径的重复度最小化。

另一方面，使用 use 引入结构体、枚举和其他项时，习惯是指定它们的完整路径。
*/

use std::collections::HashMap;

fn test_hashMap() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
/*
这个习惯用法有一个例外，那就是我们想使用 use 语句将两个具有相同名称的项带入作用域，因为 Rust 不允许这样做。
使用父模块可以区分这两个 Result 类型。
*/
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}

/*
    使用 use 将两个同名类型引入同一作用域这个问题还有另一个解决办法：在这个类型的路径后面，我们使用 as 指定一个新的本地名称或者别名。
*/

use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    // --snip--
    Ok(())
}

fn function4() -> IoResult<()> {
    // --snip--
    Ok(())
}

/*
    使用 use 关键字，将某个名称导入当前作用域后，这个名称在此作用域中就可以使用了，但它对此作用域之外还是私有的。
    如果想让其他人调用我们的代码时，也能够正常使用这个名称，就好像它本来就在当前作用域一样，那我们可以将 pub 和 use 合起来使用。
    这种技术被称为 “重导出（re-exporting）”：我们不仅将一个名称导入了当前作用域，还允许别人把它导入他们自己的作用域。
*/
mod front_of_house_5 {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house_5::hosting as hosting5;

pub fn eat_at_restaurant_5() {
    hosting5::add_to_waitlist();
}

/*
    注意 std 标准库对于你的包来说也是外部 crate。
    因为标准库随 Rust 语言一同分发，无需修改 Cargo.toml 来引入 std，
        不过需要通过 use 将标准库中定义的项引入项目包的作用域中来引用它们。
*/

/*
    当需要引入很多定义于相同包或相同模块的项时，为每一项单独列出一行会占用源码很大的空间。
    我们可以使用嵌套路径将相同的项在一行中引入作用域。
    这么做需要指定路径的相同部分，接着是两个冒号，接着是大括号中的各自不同的路径部分。

    我们可以在路径的任何层级使用嵌套路径，这在组合两个共享子路径的 use 语句时非常有用。
    use std::io;
    use std::io::Write;  ==========>>   use std::io::{self, Write};
*/

/*
    如果希望将一个路径下 所有 公有项引入作用域，可以指定路径后跟 *，glob 运算符。
    这个 use 语句将 std::collections 中定义的所有公有项引入当前作用域。
    使用 glob 运算符时请多加小心！Glob 会使得我们难以推导作用域中有什么名称和它们是在何处定义的。

    glob 运算符经常用于测试模块 tests 中，这时会将所有内容引入作用域。
    glob 运算符有时也用于 prelude 模式。
*/

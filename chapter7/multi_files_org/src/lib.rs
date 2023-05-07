mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}


/*
    我们将各个模块的代码移动到独立文件了，同时模块树依旧相同。
    eat_at_restaurant 中的函数调用也无需修改继续保持有效，即便其定义存在于不同的文件中。
    这个技巧让你可以在模块代码增长时，将它们移动到新文件中。

    注意，src/lib.rs 中的 pub use crate::front_of_house::hosting 语句是没有改变的，在文件作为 crate 的一部分而编译时，use 不会有任何影响。
    mod 关键字声明了模块，Rust 会在与模块同名的文件中查找模块的代码。
*/

/*
    Rust 提供了将包分成多个 crate，将 crate 分成模块，以及通过指定绝对或相对路径从一个模块引用另一个模块中定义的项的方式。
    你可以通过使用 use 语句将路径引入作用域，这样在多次使用时可以使用更短的路径。
    模块定义的代码默认是私有的，不过可以选择增加 pub 关键字使其定义变为公有。
*/
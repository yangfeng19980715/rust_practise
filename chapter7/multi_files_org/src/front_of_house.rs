pub mod hosting ;

/*
    注意你只需在模块树中的某处使用一次 mod 声明就可以加载这个文件。
    一旦编译器知道了这个文件是项目的一部分（并且通过 mod 语句的位置知道了代码在模块树中的位置），
        项目中的其他文件应该使用其所声明的位置的路径来引用那个文件的代码，
        这在“引用模块项目的路径”部分有讲到。
        换句话说，mod 不是 你可能会在其他编程语言中看到的 "include" 操作。
*/
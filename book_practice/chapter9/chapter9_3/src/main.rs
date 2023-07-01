/*
那么，该如何决定何时应该 panic! 以及何时应该返回 Result 呢？
如果代码 panic，就没有恢复的可能。
你可以选择对任何错误场景都调用 panic!，不管是否有可能恢复，不过这样就是你代替调用者决定了这是不可恢复的。
选择返回 Result 值的话，就将选择权交给了调用者，而不是代替他们做出决定。
调用者可能会选择以符合他们场景的方式尝试恢复，或者也可能干脆就认为 Err 是不可恢复的，所以他们也可能会调用 panic!
并将可恢复的错误变成了不可恢复的错误。因此返回 Result 是定义可能会失败的函数的一个好的默认选择。
*/
use std::net::IpAddr;

fn test_func1() {
        let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
    /*
    我们实现了一个借用了 self 的方法 value，它没有任何其他参数并返回一个 i32。
    这类方法有时被称为 getter，因为它的目的就是返回对应字段的数据。
    这样的公有方法是必要的，因为 Guess 结构体的 value 字段是私有的。
    私有的字段 value 是很重要的，这样使用 Guess 结构体的代码将不允许直接设置 value 的值：调用者 必须 使用 Guess::new 方法来创建一个 Guess 的实例，
    这就确保了不会存在一个 value 没有通过 Guess::new 函数的条件检查的 Guess。
    */

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {

}

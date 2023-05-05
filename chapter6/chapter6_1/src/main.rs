fn main() {
    /*
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    */
    /*
       // 我们可以使用一种更简洁的方式来表达相同的概念，仅仅使用枚举并将数据直接放进每一个枚举成员而不是将枚举作为结构体的一部分。
       // IpAddr 枚举的新定义表明了 V4 和 V6 成员都关联了 String 值：
       enum IpAddr {
           V4(String),
           V6(String),
       }

       // 我们直接将数据附加到枚举的每个成员上，这样就不需要一个额外的结构体了。这里也很容易看出枚举工作的另一个细节：每一个我们定义的枚举成员的名字也变成了一个构建枚举的实例的函数。
       // 也就是说，IpAddr::V4() 是一个获取 String 参数并返回 IpAddr 类型实例的函数调用。作为定义枚举的结果，这些构造函数会自动被定义。
       let home = IpAddr::V4(String::from("127.0.0.1"));

       let loopback = IpAddr::V6(String::from("::1"));
    */

    //  用枚举替代结构体还有另一个优势：每个成员可以处理不同类型和数量的数据。
    //  IPv4 版本的 IP 地址总是含有四个值在 0 和 255 之间的数字部分。如果我们想要将 V4 地址存储为四个 u8 值而 V6 地址仍然表现为一个 String，这就不能使用结构体了。枚举则可以轻易的处理这个情况：

    /*
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // 注意虽然标准库中包含一个 IpAddr 的定义，仍然可以创建和使用我们自己的定义而不会有冲突，因为我们并没有将标准库中的定义引入作用域。
    */

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // 结构体和枚举还有另一个相似点：就像可以使用 impl 来为结构体定义方法那样，也可以在枚举上定义方法。这是一个定义于我们 Message 枚举上的叫做 call 的方法：
    impl Message {
        fn call(&self) {
            // 在这里定义方法体
            println!("yes");
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // 换句话说，在对 Option<T> 进行运算之前必须将其转换为 T。通常这能帮助我们捕获到空值最常见的问题之一：假设某值不为空但实际上为空的情况。
    // 只要一个值不是 Option<T> 类型，你就 可以 安全的认定它的值不为空。这是 Rust 的一个经过深思熟虑的设计决策，来限制空值的泛滥以增加 Rust 代码的安全性。


}

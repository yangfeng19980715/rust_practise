#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        /*
        如果分支代码较短的话通常不使用大括号，正如示例 6-3 中的每个分支都只是返回一个值。
        如果想要在分支中运行多行代码，可以使用大括号，而分支后的逗号是可选的。
        */
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

/*
    ust 中的匹配是 穷尽的（exhaustive）：必须穷举到最后的可能性来使代码有效。
*/
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn add_fancy_hat() {
    println!("add fancy hat!!");
}
fn remove_fancy_hat() {
    println!("remove fancy hat ...~");

}
fn move_player(num_spaces: u8) {
    println!("you will move {} steps", num_spaces);
}

fn main() {
    let coin_penny = Coin::Penny;

    let coin_quarter = Coin::Quarter(UsState::Alaska);

    println!("{}", value_in_cents(coin_penny));
    println!("{}", value_in_cents(coin_quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("hello, world");

    let dice_roll = 9;
    /*
    请注意，我们必须将通配分支放在最后，因为模式是按顺序匹配的。
    如果我们在通配分支后添加其他分支，Rust 将会警告我们，因为此后的分支永远不会被匹配到。
    */
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    /*
    Rust 还提供了一个模式，当我们不想使用通配模式获取的值时，请使用 _ ，这是一个特殊的模式，可以匹配任意值而不绑定到该值。
    这告诉 Rust 我们不会使用这个值，所以 Rust 也不会警告我们存在未使用的变量。
    */

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => move_player(0),
    }
    /*
    最后，让我们再次改变游戏规则，如果你掷出 3 或 7 以外的值，你的回合将无事发生。
    我们可以使用单元值（在“元组类型”一节中提到的空元组）作为 _ 分支的代码
     */

    let dice_roll = 7;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

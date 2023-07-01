fn main() {
    let a = [9, 8, 7, 6, 5];

    println!("please input an array index:");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("failed to input");

    let index: usize = index.trim().parse().expect("not a number");

    let element = a[index];

    println!("a[{index}] is {element}");

    another_func(5);

    let y = {
        let x = 3;
        x + 1
    };

    println!("y = {y}");

    let ret_val = get_five();

    let ret_val = plus_one(ret_val);

    println!("ret_val is {ret_val}");
}

fn plus_one(x : i32) -> i32 {
    x + 1
}


fn get_five() -> i32 {
    5
}

fn another_func(x : i32) {
    println!("another function, x: {x}");
}

fn main() {
    let a = [9, 8, 7, 6, 5];

    println!("please input an array index:");

    let mut index = String::new();

    std::io::stdin().read_line(&mut index).expect("failed to input");

    let index: usize = index.trim().parse().expect("not a number");

    let element = a[index];

    println!("a[{index}] is {element}");

    another_func();
}

fn another_func() {
    println!("another function");
}

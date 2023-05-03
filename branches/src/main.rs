fn main() {
    /*
    let number: i32 = 3;

    if number > 0 {
        println!("positive");
    } else if number == 0 {
        println!("zero")
    } else {
        println!("negative");
    }

    let condition: bool = true;

    let value = if condition { 5 } else { 6 };

    println!("value is {value}");

    let mut loop_cnt = 1;
    let result = loop {
        println!("again, loop_cnt: {loop_cnt}");
        loop_cnt = loop_cnt + 1;

        if loop_cnt == 10 {
            break loop_cnt * 2;
        }
    };

    println!("result is {result}"); */

/*     // 循环标签和嵌套循环
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    */

    // while 循环
    let mut number = 3;

    while number != 0 {
        println!("number is {number}");
        number -= 1;
    }


    let arr : [i32; 5] = [15, 4, 3, 2, 1];
    let mut index = 0;

    while index < 5 {
        println!("arr[{index}] = {}", arr[index]);
        index += 1;
    }

    for element in arr {
        println!("element is {element}");
    }

    for n in (1..5).rev() {
        println!("{n}");
    }

    for n in 6..10 {
        println!("{n}");
    }


    println!("main end");
}

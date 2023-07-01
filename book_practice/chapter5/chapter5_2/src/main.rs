fn main_1() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_1(width1, height1)
    );
}

fn area_1(width: u32, height: u32) -> u32 {
    width * height
}

fn main_2() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_2(rect1)
    );
}

fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("Rectangle : {:#?}", rect1); // {:?} 或者 {:#?}

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}

fn area(rectangle: &Rectangle) -> u32 {
    // 注意，访问对结构体的引用的字段不会移动字段的所有权
    rectangle.width * rectangle.height
}

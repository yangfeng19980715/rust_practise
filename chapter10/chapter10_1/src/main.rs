fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

struct Point_2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point_2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point_2<X2, Y2>) -> Point_2<X1, Y2> {
        Point_2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn test_func3() {
    let p1 = Point_2 { x: 5, y: 10.4 };
    let p2 = Point_2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn test_func1() {
    let p = Point { x: 5.0, y: 10.0 };

    println!("p.x = {}", p.x());
    println!("p.distance_from_origin(): {}", p.distance_from_origin())
}

fn main() {
    test_func1();
    println!("=========================");

    test_func2();
    println!("=========================");

    test_func3();
}

fn test_func2() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

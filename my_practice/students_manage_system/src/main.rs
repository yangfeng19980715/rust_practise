
#[derive(Debug)]
struct Student{
    id : u64,
    name : String,
    age : u32
}

impl Student {
    fn new() -> Student {
        Student { id: 0u64, name: String::new(), age: 0u32 }
    }
}

fn main() {

    let entity = Student {
        id : 1001u64,
        name : "Bob".to_string(),
        age : 17u32
    };

    println!("{:?}", entity);
}

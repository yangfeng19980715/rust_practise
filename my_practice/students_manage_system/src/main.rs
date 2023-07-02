

struct StudentManager {
    list: Vec<Student>
}

impl<'a> StudentManager {
    fn add_student(&mut self, stu : Student) {
        self.list.push(stu);
    }

    fn get_size(self) -> usize {
        self.list.len()
    }

    fn query_student_by_id(self: &'a Self, id : u64) -> Option<&'a Student> {
        self.list.iter().find(|x| x.id == id)
    }
}


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

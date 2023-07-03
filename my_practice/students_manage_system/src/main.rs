#[derive(Debug)]
struct StudentManager {
    list: Vec<Student>
}

impl<'a> StudentManager {

    fn new() -> Self {
        StudentManager { list: vec![] }
    }

    fn add_student(&mut self, stu : Student) {
        //  这里可以用 any 和 find
        if self.list.iter().filter(|&item| item.id == stu.id).count() > 0 {
            println!("the id of the student exists, insert failed!");
        }
        else {
            self.list.push(stu);
        }

    }

    fn get_count(&self) -> usize {
        self.list.len()
    }

    fn query_student_by_id(self: &'a Self, id : u64) -> Option<&'a Student> {
        self.list.iter().find(|x| x.id == id)
    }

    fn modify_student_by_id(self : &mut Self, id : u64, new_val : &Student) -> bool {
        if let Some(stu) = self.list.iter_mut().find(|x| x.id == id) {
            *stu = new_val.clone();
            return true;
        }
        false
    }

    fn delete_student_by_id(self : &mut Self, id : u64) -> Option<Student> {
        let mut idx : usize = self.list.len();
        for (index, ref stu) in self.list.iter().enumerate() {
            if stu.id == id {
                idx = index;
                break;
            }
        }

        let ret = if idx < self.list.len() { Some(self.list.remove(idx))} else { None };
        ret
    }
}

#[derive(Debug, Clone)]
struct Student{
    id : u64,
    name : String,
    age : u32
}


impl Student {
    fn new(id_ : u64, name_ : String, age_ : u32) -> Student {
        Student {
            id: id_,
            name: name_,
            age: age_,
        }
    }

}

fn main() {

    let mut stuMan = StudentManager::new();

    // add
    stuMan.add_student(Student { id: 1u64, name: "Bob1".to_string(), age: 18u32 });
    stuMan.add_student(Student::new(2u64, "Bob2".to_string(), 18u32));
    stuMan.add_student(Student { id: 3u64, name: "Bob3".to_string(), age: 18u32 });

    println!("\n{:?}", &stuMan);
    println!("count of students: {}", stuMan.get_count());

    // delete
    stuMan.delete_student_by_id(1u64);
    println!("\n{:?}", &stuMan);
    println!("count of students: {}", stuMan.get_count());

    // update
    stuMan.modify_student_by_id(2, &Student::new(2u64, "John".to_string(), 31));
    println!("\n{:?}", &stuMan);
    println!("count of students: {}", stuMan.get_count());

    // query
    let ret = stuMan.query_student_by_id(3);
    println!("\nret: {:?}", ret);
    println!("{:?}", &stuMan);
    println!("count of students: {}", stuMan.get_count());

}


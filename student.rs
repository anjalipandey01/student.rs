struct Student{
    name: String,
    email: String,
    phno: String,
    id: u32,
}
fn main() {
    let mut students:Vec<Student>=Vec::new();
    students.push(Student{
        name: String::from("Suman"),
        email: String::from("suman@gmail.com"),
        phno: String::from("987654321"),
        id: 5,
    });

    students.push(Student{
        name: String::from("Alice"),
        email: String::from("alice@gmail.com"),
        phno: String::from("678945321"),
        id: 8,
    });

    students.push(Student{
        name: String::from("George"),
        email: String::from("george@gmail.com"),
        phno: String::from("6777443521"),
        id: 3,
    });

    students.push(Student{
        name: String::from("Martin"),
        email: String::from("martin@gmail.com"),
        phno: String::from("124537689"),
        id: 2,
    });

    students.push(Student{
        name: String::from("Jerry"),
        email: String::from("jerry@gmail.com"),
        phno: String::from("6678594123"),
        id: 9,
    });

    let index=3;
    match students.get(index) {
        Some(student) => {
            println!("Student {}:\n Name: {}\n Email: {}\n Phone: {}\n Id: {}",
                    index,
                    student.name,
                    student.email,
                    student.phno,
                    student.id);
        }
        None => {
            println!("Student at index {} not found. ",index);
        }
    }
}

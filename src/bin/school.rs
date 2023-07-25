#[allow(clippy::new_without_default)]
#[derive(Debug)]
pub struct School {
    students: Vec<Student>,
}
#[derive(Clone, Debug)]
pub struct Student {
    name: String,
    grade: u32,
}

impl School {
    pub fn new() -> School {
        School { students: vec![] }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.push(Student {
            name: String::from(student),
            grade,
        })
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = Vec::new();
        for student in self.students.clone() {
            if !grades.contains(&student.grade) {
                grades.push(student.grade)
            }
        }
        grades.sort();
        grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students: Vec<String> = Vec::new();
        for student in self.students.clone() {
            if student.grade == grade {
                students.push(student.name)
            }
        }
        students.sort();
        students
    }
}

fn main() {
    let student = Student {
        name: String::from("Pedro"),
        grade: 8,
    };
    let student2 = Student {
        name: String::from("Lucas"),
        grade: 8,
    };
    let mut students_vec: Vec<Student> = [].to_vec();
    students_vec.push(student);
    students_vec.push(student2);
    let mut school = School {
        students: students_vec,
    };
    school.add(8, "Leoti");
    let school2 = School::new();
    println!("School: {:?}", school.grade(8));
    println!("School: {:?}", school2.grade(8))
}

// many students belong to many courses

struct Student {
    name: String
}

impl Student {
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform.enrollments.iter()
        .filter(|&e|
            e.student.name == self.name)
            .map(|e| e.course.name.clone())
                .collect()
    }
}

struct Course {
    name: String
}

struct Enrollment<'a> {
    student: &'a Student,
    course: &'a Course
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
        Enrollment {student, course }
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>
}

impl<'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new()
         }
    }

    fn enroll(&mut self, student: &'a Student, course: &'a Course) {
        self.enrollments.push(
            Enrollment::new(student, course)
        )
    }
}

pub fn cref(){
    let muis =  Student {name: "Muis".into()};
    let course =  Course {name: "Running from cats".into()};

    let  mut p =  Platform::new();
    p.enroll(&muis, &course);

    for c in muis.courses(p){
        println!("Muis is taking {}",c)
    }
}

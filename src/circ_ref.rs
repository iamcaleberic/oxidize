use std::rc::Rc;
use std::cell::RefCell;

// many students belong to many courses

struct Student {
    name: String,
    courses: Vec<Rc<RefCell<Course>>>
}


struct Course{
    name: String,
    students: Vec<Rc<RefCell<Student>>>
}

impl Student {
    fn new(name: &str) -> Student {
        Student {
            name: name.into(),
            courses: Vec::new()
        }
    }
}

impl Course {
    fn new(name: &str) -> Course {
        Course {
            name: name.into(),
            students: Vec::new()
        }
    }

    fn add_student(course: Rc<RefCell<Course>>, student: Rc<RefCell<Student>> ) {
        student.borrow_mut().courses.push(course.clone());
        course.borrow_mut().students.push(student.clone());
        course.borrow_mut().students.push(student.clone());
    }
}

pub fn cref(){
    let muis =  Rc::new(RefCell::new(Student::new("Muis")));
    let kat =  Rc::new(RefCell::new(Student::new("kat")));

    let course =  Course::new("AstroSomething");
    let magic_course = Rc::new(RefCell::new(course));

    // course.add_student(muis);
    Course::add_student(magic_course.clone(), muis);
    Course::add_student(magic_course, kat);
}

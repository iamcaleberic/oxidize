use std::rc::Rc;

// ref count variables

struct Person {
    name: Rc<String>
}

impl Person {
    fn new(name: Rc<String>) -> Person {
        Person{name: name}
    }

    fn greet(&self){
        println!("Hi my name is {}", self.name);
    }
}

pub fn rc(){
    let name = Rc::new("Dal".to_string());
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    {
        let person = Person::new(name.clone());
        person.greet();
        println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    }
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));

    println!("Name = {}", name)
}

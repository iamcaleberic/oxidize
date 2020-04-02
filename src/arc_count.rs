use std::sync::{Mutex,Arc};
use std::thread;
// ref count variables

struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>

}

impl Person {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
        Person{name: name, state: state}
    }

    fn greet(&self){
        let mut state = self.state.lock().unwrap();

        state.clear();
        state.push_str("Happy");

        println!("Hi my name is {}. and I am {}", self.name, state.as_str());
    }
}

pub fn arc(){
    let name = Arc::new("Dal".to_string());
    let state = Arc::new(Mutex::new("Bored".to_string()));
    let person = Person::new(name.clone(), state.clone());
    let t = thread::spawn(move || {
        person.greet();
    });

    println!("Name = {} State = {}", name, state.lock().unwrap().as_str());

    t.join().unwrap();


}

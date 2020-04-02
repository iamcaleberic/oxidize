
// static lifetime is the longest lifetime possible
// lasting for the dur of a program
struct Person {
    name: String
}

impl Person {
    // fn get_ref_name(&self) -> &String {
    fn get_ref_name<'a>(&'a self) -> &'a String {
        &self.name
    }
}

struct Company<'z> {
    name: String,
    ceo: &'z Person
}


pub fn lf(){
    let p = Person{ name: String::from("Lem")};
    let c = Company{ name: String::from("Unknown"), ceo: &p};

    let  z: &String;
    {
        let p = Person{ name:  String::from("Doe")};
        p.get_ref_name();
    }
}

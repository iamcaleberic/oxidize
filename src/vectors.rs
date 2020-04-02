trait Animal {
    // fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;

    fn talk(&self){
        println!("{} cannot talk", self.name())
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    // fn create(name: &'static str) -> Human {
    //     Human{name: name}
    // }
    fn name(&self) ->  &'static str {
        self.name
    }

    fn talk(&self){
        println!("{} says Hello :)", self.name())
    }
}

impl Animal for Cat {
    // fn create(name: &'static str) -> Cat {
    //     Cat{name: name}
    // }

    fn name(&self) ->  &'static str {
        self.name
    }

    fn talk(&self){
        println!("{} says Meeow", self.name())
    }
}

enum Creature {
    Human(Human),
    Cat(Cat)
}

pub fn vectors(){
    let mut vc = Vec::new();
    vc.push(56);
    vc.push(21);
    vc.push(88);

    println!("vc = {:?}", vc);
    println!("vc[2] = {}", vc[2]);

    match vc.get(2){
        Some(x) => println!("vc[2] = {}", x),
        None => println!("index out of range")
    };

    for x in &vc { println!("{}", x); };

    vc.push(29);

    let last_elem =  vc.pop();

    println!("last elem is {:?}, vc = {:?}", last_elem, vc);

    while let Some(x) = vc.pop(){
        println!("{}", x)
    }

    let mut creatures  = Vec::new();

    creatures.push(
        Creature::Human(Human { name: "Adem Halen"})
    );

    creatures.push(
        Creature::Cat(Cat {name: "Ogli"})
    );

    for c in creatures {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(ct) => ct.talk()
        }
    }

    let mut animals:Vec<Box<dyn Animal>> = Vec::new();

    animals.push(Box::new(Human{name: "Spin"}));
    animals.push(Box::new(Cat{name: "Asin "}));

    for a in animals.iter() {
        a.talk()
    }
}

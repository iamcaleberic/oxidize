use std::fmt::Debug;

trait Animal {  
    fn create(name: &'static str) -> Self;
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
    fn create(name: &'static str) -> Human {
        Human{name: name}
    }
    fn name(&self) ->  &'static str {
        self.name
    }

    fn talk(&self){
        println!("{} says Hello :)", self.name())
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat{name: name}
    }

    fn name(&self) ->  &'static str {
        self.name
    }

    fn talk(&self){
        println!("{} says Meeow", self.name())
    }
}


trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>{
    fn sum(&self) -> i32 {
        let mut result = 0;
        for x in self {result += *x; }

        return result;
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Square {
    side: f64,
}


trait Shape {
    fn area(&self) -> f64;
}


impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

// Defining funcs that take trait params

// fn print_info(shape: impl Shape + Debug){
// fn print_info<T: Shape + Debug>(shape: T)
fn print_info<T>(shape: T) where T: Shape + Debug {
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}


pub fn traits(){
    // let h = Human{name: "Vlinder"};
    // let h = Human::create("Vlinder");
    let h:Human = Animal::create("Vlinder");
    h.talk();

    let c = Cat{name: "Berg"};
    c.talk();

    let vc =  vec![1,2,3];
    println!("sum = {}", vc.sum());

    let c =  Circle{ radius: 12.0};
    print_info(c)
}
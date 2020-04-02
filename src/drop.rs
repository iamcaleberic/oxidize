struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature{
        println!("{} enters the game", name);
        Creature {name: name.into()}
    }
}

impl Drop for Creature {
    fn drop(&mut self){
        println!("{} is dead", self.name);
    }
}

pub fn drop(){
    let goblin = Creature::new("Gobbles");
    println!("game proceeds");
}

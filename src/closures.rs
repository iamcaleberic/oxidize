fn say_hello(){ println!("Hello:)");}

pub fn closures(){
    let sh = say_hello;
    sh();

    let plus_one = |x: i32| { x + 1 };

    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a) );

    let mut two =  2;
    {
        let plus_two = |x| {
            let mut z = x;
            z += 2; 
            z
        };
        println!("{} + 2 = {}", 7 , plus_two(7));
    };

    let borrow_two = &mut two;


    // T: by value 
    // T&
    // &mut &

    let plus_three = |x: &mut i32| *x += 3;
    let mut f = 12; 

    plus_three(&mut f);
    println!("f = {}",f);

}
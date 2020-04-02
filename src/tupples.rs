fn sum_and_product(x: i32, y: i32) -> (i32,i32) {
    (x+y, x*y)
}

pub fn tupples(){
    let x = 5;
    let y = 8;
    let sp = sum_and_product(x,y); 

    println!("{:?}", sp);

    println!("{0} + {1} ={2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);  

    let sp2 =  sum_and_product(4, 7);
    let combined = (sp, sp2);

    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c,d), (e,f)) =  combined;

    let foo =  (true , 50.1 , -9i8);
    println!("{:?}", foo);

    let sint = (45,);
    println!("{:?}", sint);
}
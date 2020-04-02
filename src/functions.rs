struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn print_value(x: i32){
    println!("Value is {}", x);
}

fn increase(x: &mut i32){
    *x += 1;
}

fn product(x: i32 , y:i32) -> i32 {
    x * y 
}

pub fn functions(){
    print_value(34);

    let mut z = 1;

    increase(&mut z);
    println!("z = {}", z );

    let a = 6;
    let b = 9; 
    let p = product(a,b);

    println!("{} * {} = {}", a, b, p);

    let p = Point{ x: 7.0 , y: 4.0};
    let pl = Point {x: 3.0, y: 9.0};
    let myline = Line {start: p , end: pl };

    println!("lenght of line is: {}", myline.len());
}


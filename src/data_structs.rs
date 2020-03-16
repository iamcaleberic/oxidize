struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

fn structures(){
    let p = Point{ x: 7.0 , y: 4.0};
    println!("point is ({}, {})", p.x, p.y);

    let pl = Point {x: 3.0, y: 9.0};
    let myline = Line {start: p , end: pl };
}


pub fn data_structs() {
    structures();
}
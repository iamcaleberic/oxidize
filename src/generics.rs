struct Point<T,V>{
    x: T,
    y: V
}


pub fn generics(){
    let a:Point<i32, u32> =  Point {x: 0, y: 0};
    let b:Point<f64, i64> =  Point {x: 1.2,y: 300}; 
}
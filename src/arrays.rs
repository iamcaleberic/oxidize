pub fn arrays(){
    let mut a:[i32;5] = [1, 2,3 ,4,5];

    println!("a has {}, first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("a[0] is {}", a[0]);

    println!("{:?}", a);
}
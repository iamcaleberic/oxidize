pub fn arrays(){
    let mut a:[i32;5] = [1, 2,3 ,4,5];

    println!("a has {}, first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("a[0] is {}", a[0]);

    println!("{:?}", a);
    if a != [1,2,3,4,5] {
        println!("Does not match");
    }

    let b = [1u16; 10];
    for i in 0..b.len(){
        println!("{}", b[i]);
    }

    let mtx:[[f32;3]; 2] = [
        [0.5, 0.8, 0.7],
        [0.9, 0.1, 0.0]
    ];

    println!("{:?}", mtx);

    for i in 0..mtx.len(){
        for j in 0..mtx[i].len(){
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j])
            }
        }
    }
}
fn use_slice(slice: &mut[i32]){
    println!("first element = {}, len = {}", slice[0], slice.len());

    slice[0] = 1631;
}

pub fn slices(){
    let mut sl = [1,4,5,8,43, 51];

    // use_slice(&mut sl[1..4]);
    use_slice(&mut sl);

    println!("{:?}", sl)
}
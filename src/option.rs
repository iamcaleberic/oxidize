

pub fn option(){
    let x = 4.0;
    let y = 1.0;

    //Option => Some(v) | None

    let res =  
        if y != 0.0 { Some(x/y) } else {None};
    
    match res {
        Some(z) => println!("{}/{}={}",x,y,z),
        None => println!("cannot divide by zero")
    }

    if let Some(z) = res {
        println!("result = {}",z)
    }
}
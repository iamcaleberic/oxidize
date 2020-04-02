use std::collections::HashMap;

pub fn hash_map(){
    let mut shapes  = HashMap::new();

    shapes.insert(String::from("triangle"),3);
    shapes.insert(String::from("square"),3);

    println!("a square has {} sides", shapes["square".into()]);

    
    for (key, value) in &shapes {
        println!("{} : {}", key, value)
    };

    shapes.insert("square".into(), 4);

    shapes.entry("circle".into()).or_insert(1);

    
    {
        let actual = shapes.entry("circle".into()).or_insert(2);  
        *actual = 0;
    }
    
    println!("{:?}", shapes);
}
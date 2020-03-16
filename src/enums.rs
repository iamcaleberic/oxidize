enum Color{
    Red, 
    Green, 
    Blue, 
    RgbColour(u8,u8,u8), // tuple like def
    CmykColour{cyan:u8, magenta:u8 , yellow:u8, black:u8} //struct like def
}

pub fn enums (){
    let c:Color = Color::CmykColour{cyan: 0, magenta: 133, yellow: 0, black: 255};

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColour(0,0,0) 
        | Color::CmykColour{cyan: _, magenta:_, yellow:_, black:255} => println!("black"),
        Color::RgbColour(r,g,b) => println!("rgb({},{},{})", r,g,b),
        _ => println!("Unknown Color")
    }
}
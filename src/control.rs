fn if_statement() {
    let speed = 140;

    if speed > 130 {
        println!("Toooo fast");
    } else if speed < 130 {
        println!("All good");
    }

    println!(
        "You are {}",
        if speed > 130 {
            "Too fast"
        } else if speed < 130 {
            "All Good"
        } else {
            "Either stopped or Supersonic"
        }
    )
}

fn while_n_loop() {
    let mut x = 2;
    while x < 1000 {
        x *= 2;
        if x == 128 {
            continue;
        }
        println!("x is {}", x);
    }

    let mut y = 1;
    loop {
        y *= 2;
        println!("y is {}", y);

        if y == 1 << 10 {
            break;
        };
    }
}

fn match_statement() {
    let country_code = 999;

    let country = match country_code {
        31 => "Nederland",
        254 => "Kenia",
        32 => "Belgie",
        1..=999 => "unknown",
        _ => "invalid"
    };

    println!("The country with code {} is {}", country_code, country)
}

fn for_loop() {
    for b in 1..11 {
        println!("b is {}", b);
        if b == 2 {
            continue;
        }
        if b == 8 {
            break;
        }
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{} is {}", pos, y)
    }
}

pub fn control_flow() {
    if_statement();
    while_n_loop();
    for_loop();
    match_statement();
}

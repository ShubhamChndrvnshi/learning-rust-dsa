struct Color(u8, u8, u8);

fn main() {
    
    let mut red: Color = Color(255, 0, 0);

    red.0 = 250;

    println!("red is {}, {}, {}", red.0, red.1, red.2);

}

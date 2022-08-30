## Structs

- Way to group similar bit of information.
- Structs can have hetrogeneous data type variables.
Ex:
```
struct Color {
    red: u8,
    green: bool,
    blue: u8
}

fn main() {
    
    let mut bg: Color = Color { red: 255, green: true, blue: 15 };

    bg.blue = 244;

    println!("{}, {}, {}", bg.red, bg.green, bg.blue);
}
```


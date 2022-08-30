## varibale data types in Rust using Cargo

Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data.
https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html

Ex:
```
fn main() {
    let x = 45; // internally rust knows it's i32 type
    let y: i64 = 65;
    let z: u64 = 8;

    let f: f32 = 6.7; // f32

    let b: bool = false;
}
```

If value of any values overflows, the program thows error ( Unrecoverable Errors with Panic). When program is running using <b>--release</b> flag. Rust uses two’s complement wrapping. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on. 
## Variables

- We can create a variable in rust using <b> let </b> keyword.
Ex:
```
fn main() {
    let x = 45;
    println!("The value of x is {}", x);
}
```

- Point to note here is: All variables in Rust is immutable by default, that's why below code will give error.
```
fn main() {
    let x = 45;

    println!("The value of x is {}", x);

    x = 60;

    println!("The value of x is {}", x);

    /*
  shows nice error message
  ------
 error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:6:5
  |
2 |     let x = 45;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
...
6 |     x = 60;
  |     ^^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` due to previous error
  ------
*/
}
```

- To make a variable mutable, we can use <b>mut</b> keyword.
```
fn main() {
    let mut x = 45;

    println!("The value of x is {}", x);

    x = 60;

    println!("The value of x is {}", x);
}
```
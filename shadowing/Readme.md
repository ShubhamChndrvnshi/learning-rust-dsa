## Shadowing

- Variable bindings have a scope, and are constrained to live in a block.
- This is usefull when you want the chages done to a certain variables to be restricted to that code block only.

Ex:
```
fn main(){
    let mut x: u32 = 10;

    {
        let x = 15;

        // DO something of it
    }

    println!("Value of x is unchanged x:{}", x);
}
```
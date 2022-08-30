## References

- Not similar to c/c++ references.
- We can access the reference of a variable using <b>&</b> keyword.
- Here we are just giving a different reference to a variable, like a pet name for a person.
Ex:
```
fn main(){
    let x: u32 = 10;
     let xr = &x;

    println!("Value of xr is x:{}", xr);
}
```

- At any given time, you can have either one mutable reference or any number of immutable references.

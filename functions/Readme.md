## Functions

- We define a function in Rust by entering <b>fn</b> followed by a function name and a set of parentheses. The curly brackets tell the compiler where the function body begins and ends.
- They are defined as follows:
Ex:
```
fn main() {

    print_numbers_to(10);
}

fn print_numbers_to(num: u32) {
    println!("Printing nu mbers from 1 to {}", num);

    for n in 1..(num+1) {
        if is_even(n) {
            println!("{} is even",n);
        } else {
            println!("{} is odd",n);
        }
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}
```
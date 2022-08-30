fn main() {

    print_numbers_to(10);
}

fn print_numbers_to(num: u32) {
    println!("Printing numbers from 1 to {}", num);

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
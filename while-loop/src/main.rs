fn main() {
    println!("Print multiples of 5");

    let mut n = 1;

    while n <=50 {
        // if n is a multiple of 5
        if n % 5 == 0{
            println!("n is {}", n);
        }
        
        n += 1;

    }
}

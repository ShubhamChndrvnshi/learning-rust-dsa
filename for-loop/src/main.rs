fn main() {
    println!("Iterating over range");

    let numbers = 30..51;

    for i in numbers {
        println!("The i is {}", i);
    }

    println!("Iterating over vector");

    let animals = vec!["Rabbit", "Dog", "Cart"];

    for a in animals.iter() {
        println!("The animal is {}", a);
    }

    // To get index
    for (index, a) in animals.iter().enumerate() {
        println!("The index is {} and the animal is {}", index, a);
    }
}

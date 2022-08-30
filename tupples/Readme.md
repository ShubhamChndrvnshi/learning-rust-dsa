## Tupples

- Couple of variables in once collection or in one sub-space.
- They are defined as follows:
Ex:
```
fn main(){

    let tup1 = (20, 25, 35);
}
``  

- Tupple can store heterogeneous data types
- Tupples can also store tupples i.e, nested tupples.
- Nested tupples can be accessed as follows:
```
fn main() {

    let tup1 = (20, 25, 35, "India", (1,2,3));

    println!("{}", (tup1.4).1);
}
```

- Destructuring tupples
```
fn main(){
    let tup1 = (45, 65.7, "Computer");

    let (a, b, c) = tup1;

    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);
}
```
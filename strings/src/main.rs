fn main() {
    let mut str = String::from("How are you");

    //length 
    println!("Print the length of string: {}", str.len());

    // is empty string
    println!("String is empty: {}",str.is_empty());

    // splitting strings

    for token in str.split_whitespace() {
        println!("{}",token);
    }

    // check for sub string

    println!("My string contains 'are' : {}", str.contains("are"));

    // append

    str.push_str(" today ?");

    println!("{}", str);
}

fn main(){
    let x: u32 = 10;

    {
        let x: u8 = 15;

        // DO something of it
    }
    let x = true;
    println!("Value of x is unchanged x:{}", x);
}
fn main(){
    
    let mut x = 10;

    let xr = &mut x;

    *xr +=1;

    println!("Value of xr is {}", xr);

    println!("Value of x is {}", x);
}
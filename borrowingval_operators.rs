fn main(){
    let a = 10;
    let c = &a;
    println!("{}",c);
    let mut b = 7;
    let d = &mut b;
    println!("{}",d);
    
    let x = 8;
    let y = 9;
    println!("x and y is  {}",x&y);
    println!("x or y is {}",x | y);
    println!("x xor y is {}",x ^ y);
    println!("not x is {}",!x);
    println!("not y is {}", !y);
    println!("Right shift of x is {}",x>>2);
    println!("left shift of y is {}",y<<2);
    
}

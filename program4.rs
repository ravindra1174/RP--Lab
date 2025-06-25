fn main(){
    let x = 5;
    println!("{}",x);//outer scope of the variable x.
    {
        let x = 6; // inner scope
        println!("{}",x); // prints 6 not 5 which is in outer scope.
    }
    println!("{}",x); // prints only 5 not 6 which is in inner scope.
    let x = x*2;
    println!("{}",x);
    let x = x - 2;
    println!("{}",x);
}

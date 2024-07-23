fn main() {
    let  x = 5;
    println!("the value of x is : {x}");
    {
        let x = x + 30;
        println!("the value of x is {x}");
    }
    let  x = x-3;
    println!("the value of x is : {x}")
    
}

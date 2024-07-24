fn main() {
    let a: i32 = another_function(1);
    println!("{}",a);
}

fn another_function(x:i32) ->i32 {
    println!("{} Another Function!",x);
    return  x + 10; //세미 콜론 없이 반환할 값을 명시 할 수 있다
}   
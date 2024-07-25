fn main(){
    let number = Some(30);
    let number_plus = plus_one(number);
    
}

fn plus_one(number:Option<i32>) -> Option<i32>{
    match number {
        Some(i) => Some(i+1),
        None => None
    }
}
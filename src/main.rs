fn main() { 
    let mut s: String = String::from("hello");

    let r1: &String = &s; // 문제없음
    let r2: &String = &s; // 문제없음
    println!("{} and {}", r1, r2);
    // 이 지점 이후로 변수 r1과 r2는 사용되지 않습니다

    let r3: &mut String = &mut s; // 문제없음
    println!("{}", r3);
}
fn main() {
    // i32 타입의 변수를 정의
    let n: i32 = 100;

    // i64 타입에 i32 타입의 값을 대입
    let m: i64 = n as i64;  // as 를 이용해 i64 타입으로 변환
    println!("{},{}", n, m);
}
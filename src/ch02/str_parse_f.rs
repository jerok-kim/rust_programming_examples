fn main() {
    // 문자열 지정
    let s = "3.1415a";

    // f64 타입으로 변환
    let num = s.parse::<f64>();

    match num {
        Ok(result) => println!("{:.2}", result),
        Err(e) => println!("에러가 발생했습니다. 이유 : '{:?}'", e)
    }
}
// 문법 규칙 시작
peg::parser!( grammar calc() for str {
    // 기본이 되는 규칙 추가
    pub rule eval() -> i64
        = term()
    
    // 덧셈을 수행하는 규칙 추가
    rule term() -> i64
        = v1:num() "+" v2:num()
        { v1 + v2 }
    
    // 숫자 값을 읽는 규칙 추가
    rule num() -> i64
        = value:$(['0'..='9']+)
        { value.parse().unwrap() }
});

fn main() {
    // 덧셈 계산식 실행
    println!("2+5={}", calc::eval("2+5").unwrap());
    println!("8+2={}", calc::eval("8+2").unwrap());
    println!("200+50={}", calc::eval("200+50").unwrap());
}
﻿fn main() {
    let pr = "지혜는 무기보다 가치가 있다.";
    
    // 앞의 2글자(6바이트)를 얻기
    println!("앞 2글자: {}", &pr[0..6]);
    
    // '무기' 부분을 얻기
    println!("4-5번째 글자: {}", &pr[10..16]);
}
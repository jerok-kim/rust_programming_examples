﻿// HashMap 초기화 매크로
macro_rules! map_init {
    ( $($key:expr => $val:expr), * ) => {{
        // HashMap 객체 생성
        let mut tmp = std::collections::HashMap::new();
        $(
            // 반복해서 값 넣기
            tmp.insert($key, $val);
        )*
        tmp  // 객체 반환        
    }}
}

fn main() {
    // 매크로로 HashMap 초기화
    let week = map_init![
        "mon" => "월요일",
        "tue" => "화요일",
        "wed" => "수요일",
        "thu" => "목요일",
        "fri" => "금요일",
        "sat" => "토요일",
        "sun" => "일요일"
    ];
    println!("mon={}", week["mon"]);
    println!("wed={}", week["wed"]);
}
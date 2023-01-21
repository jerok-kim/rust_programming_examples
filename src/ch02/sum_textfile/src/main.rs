use std::{env, fs};

fn main() {
    // 명령줄 인수 취득
    let args = env::args();
    let mut total: i32 = 0;

    // 모든 인수를 처리
    for (i, fname) in args.enumerate() {
        if i == 0 { continue; }

        // 텍스트 파일을 읽어들임
        let text = fs::read_to_string(fname).unwrap();

        // 한줄씩 분리
        let lines = text.split('\n');

        // 반복해서 계산
        for line in lines {
            // 숫자 값으로 변경
            let n: i32 = match line.parse::<i32>() {
                Ok(v) => v,
                Err(_) => 0,
            };
            total += n;
        }
    }

    // 결과 표시
    println!("{}", total);
}
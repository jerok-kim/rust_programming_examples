use std::fs;

fn main() {
    let file = "./fizzbuzz.txt";
    let str = fs::read_to_string(file);
    
    // 읽어들인 내용을 가공하지 않고 그냥 출력
    println!("RAW DATA : {:?}", str);
    
    // unwrap 메서드를 이용 후 출력
    println!("Unwarp() : {:?}", str.unwrap());
}
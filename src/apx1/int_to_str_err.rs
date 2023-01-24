// 문제가 있는 프로그램
fn int_to_str(value: i64) -> &str {
    let s = format!("{}", value);
    &s
}

fn main() {
    let s = int_to_str(256);
    println!("{}", s);
}
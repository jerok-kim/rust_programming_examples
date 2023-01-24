// 문제가 있는 프로그램
fn main() {
    let target = "aaa,bbb,ccc";
    let lines: Vec<String> = target.split(",").collect();
    println!("{:?}", lines);
}
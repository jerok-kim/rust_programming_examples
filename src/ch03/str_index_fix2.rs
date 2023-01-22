fn main() {
    // 첫 번째 글자를 출력
    let s2 = "abcdefg";
    println!("{}", &s2[0..1]);

    let s = "안녕하세요";

    // 첫 번째 1글자를 출력
    let ch = &s[..3];
    println!("{}", ch);

    // 세 번째 1글자를 출력
    let ch = &s[6..9];
    println!("{}", ch);
}
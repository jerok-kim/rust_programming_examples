fn main() {
    // 힙 영역에 i32 타입 값 1000을 저장
    let a_box = Box::new(1000);
    {
        let b_box= a_box;  // 소유권 이동
        println!("{}", b_box);
    }
    println!("{}", a_box);  // 소유권이 이동했으므로 이용 불가
}
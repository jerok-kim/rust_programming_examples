use std::rc::Rc;

fn main() {
    // 힙 영역에 i32 타입 값 1000을 저장
    let mut a_rc = Rc::new(1000);

    // a_rc의 참조를 복제
    let mut b_rc = Rc::clone(&a_rc);

    // b_rc값 변경 시도
    *b_rc += 100;  // 에러가 발생하는 부분
    println!("{}", b_rc);
}
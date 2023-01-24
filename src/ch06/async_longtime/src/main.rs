use tokio::time;

#[tokio::main]
async fn main() {
    // 비동기 처리를 연속으로 실행
    for i in 1..=3 {
        println!("#{} 시작", i);

        // 비동기 처리 함수를 실행해 결과를 얻는다
        let s = read_longtime().await;
        println!("{}", s);

        // 비동기 처리는 블록에서도 사용 가능
        let s = async {
            time::sleep(time::Duration::from_secs(1)).await;
            String::from("길게 읽어들이기 완료(block)")
        }.await;
        println!("{}", s);
    }
}

// 시간이 걸리는 함수
async fn read_longtime() -> String {
    time::sleep(time::Duration::from_secs(1)).await;
    String::from("길게 읽어들이기 완료(fn)")
}
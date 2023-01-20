fn main() {
    let price = 3950;
    
    for coin_500 in 0..=10 {
        for coin_100 in 0..=3 {
            for coin_50 in 0..=10 {
                if coin_500 * 500 + coin_100 * 100 + coin_50 * 50 == price {
                    println!("500원x{} + 100원x{} + 50원x{} = {}", coin_500, coin_100, coin_50, price);
                }
            }
        }
    }
}
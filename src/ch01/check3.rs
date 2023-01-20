fn main() {
    for i in 1..=50 {
        if i % 3 == 0 || (i % 10) == 3 || (i / 10) == 3 {
            println!("A");
        } else {
            println!("{}", i);
        }
    }
}
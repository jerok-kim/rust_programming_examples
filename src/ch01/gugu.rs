fn main() {
    for i in 1..=9 {
        for j in 1..=9 {
            if j == 9 {
                print!("{:3}", i * j);
                break;
            }
            print!("{:3},", i * j);
        }
        println!();
    }
}
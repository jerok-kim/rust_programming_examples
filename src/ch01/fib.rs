fn main() {
    let result = fibonacci(17);
    println!("{result}");
}

fn fibonacci(n: i32) -> i32 {
    return if n == 1 {
        1
    } else if n == 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
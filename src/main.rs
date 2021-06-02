fn main() {
    for nth in 0..11 {
        let ans = fibonacci(nth);
        println!("{}th fibonacchi ans is: {}", nth, ans);
    }
}

// Formula is: https://ja.wikipedia.org/wiki/%E3%83%95%E3%82%A3%E3%83%9C%E3%83%8A%E3%83%83%E3%83%81%E6%95%B0
fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        n
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}

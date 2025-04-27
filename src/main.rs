use scope_timer::ScopeTimer;

fn fib(number: usize) -> usize {
    match number {
        0..=1 => 1,
        _ => fib(number - 1) + fib(number - 2),
    }
}

fn main() {
    for number in 0..=41 {
        let _ = ScopeTimer::new();
        println!("fib({}) = {}", number, fib(number));
    }
}

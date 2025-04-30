pub fn solution() -> u32 {
    let mut fib = vec![1, 2];
    while fib.last().unwrap() < &4_000_000 {
        fib.push(fib.last().unwrap() + fib.get(fib.len() - 2).unwrap());
    }

    fib.iter().filter(|i| *i % 2 == 0).sum::<u32>()
}

pub fn solution() {
    let mut fib = vec![1, 2];
    while fib.last().unwrap() < &4_000_000 {
        fib.push(fib.get(fib.len() - 1).unwrap() + fib.get(fib.len() - 2).unwrap());
    }

    let yep = fib.iter().filter(|i| *i % 2 == 0).sum::<u32>();
    println!("p002: {yep}");
}

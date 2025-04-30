pub fn solution() -> u64 {
    (1..1_000_000)
        .map(count_sequance_len)
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0
}

fn count_sequance_len(n: u64) -> (u64, u64) {
    let mut num = n;
    let mut c = 0;
    while num != 1 {
        c += 1;
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = 3 * num + 1;
        }
    }
    (n, c)
}

pub fn solution() -> usize {
    let mut n = 0;
    let mut tn = 0;
    loop {
        n += 1;
        tn += n;
        if count_divisors(tn) > 500 {
            return tn;
        }
    }
}

fn count_divisors(n: usize) -> usize {
    let mut count = 0;
    for i in 1..=(n as f64).sqrt().trunc() as usize {
        if n % i == 0 {
            if n % i == i {
                count += 1;
            } else {
                count = count + 2
            }
        }
    }
    count
}

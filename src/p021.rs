pub fn solution() -> usize {
    let mut result = 0;
    for i in 1..10000 {
        let a: usize = crate::common::get_all_divisors(i).into_iter().sum();
        if i == a {
            continue;
        }
        let b: usize = crate::common::get_all_divisors(a).into_iter().sum();
        if i == b {
            result += i;
        }
    }
    result
}

use std::collections::HashMap;

use crate::common::get_all_divisors;

pub fn solution() -> u32 {
    let mut abundant_nums = HashMap::new();
    for i in 3..28124 {
        abundant_nums.entry(i).or_insert(is_abundant(i));
    }
    (1..28124)
        .filter(|i| !is_sum_abundants(&abundant_nums, *i as usize))
        .sum()
}

fn is_abundant(n: usize) -> bool {
    get_all_divisors(n).iter().sum::<usize>() > n
}

fn is_sum_abundants(abundant_nums: &HashMap<usize, bool>, n: usize) -> bool {
    if n > 28123 {
        return true;
    }

    for i in 3..n {
        if *abundant_nums.get(&i).unwrap_or(&false)
            && *abundant_nums.get(&(n - i)).unwrap_or(&false)
        {
            return true;
        }
    }
    false
}

use crate::common::is_prime;

pub fn solution() -> u64 {
    (2..2_000_000).filter(|i| is_prime(*i as usize)).sum()
}

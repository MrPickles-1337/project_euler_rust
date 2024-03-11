use num_bigint::BigUint;

pub fn solution() -> u32 {
    vec![BigUint::from(2u32); 1000]
        .iter()
        .fold(BigUint::from(1u32), |a, b| a * b)
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}

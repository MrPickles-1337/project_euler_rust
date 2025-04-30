use num_bigint::BigUint;

use crate::common;

pub fn solution() -> u64 {
    let mut f = common::factorial(100u32);
    let mut sum = BigUint::from(0u8);
    while f != BigUint::from(0u8) {
        let digit = &f % BigUint::from(10u8);
        f /= BigUint::from(10u8);
        sum += digit;
    }

    *sum.to_u64_digits().first().unwrap()
}

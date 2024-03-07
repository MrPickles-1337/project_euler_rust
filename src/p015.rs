use crate::common::factorial;

pub fn solution() -> String {
    (factorial(40u32) / (factorial(20u32) * factorial(20u32))).to_string()
}

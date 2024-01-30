use crate::common::is_prime;

pub fn solution() {
    let mut count = 0;
    let mut i = 0;
    while count < 10_001 {
        i += 1;
        if is_prime(i) {
            count += 1;
        }
    }
    println!("p007: {i}");
}

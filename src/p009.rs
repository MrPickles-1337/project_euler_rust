fn is_pythagorean_triplet(a: u32, b: u32, c: u32) -> bool {
    (a * a + b * b) == c * c || (a * a + c * c) == b * b || (b * b + c * c) == a * a
}
pub fn solution() -> u32 {
    for a in 1..334 {
        for b in (a..1000 - a).step_by(3) {
            let c = 1000 - a - b;
            if c < 1 {
                break;
            }
            if is_pythagorean_triplet(a, b, c) {
                return a * b * c;
            }
        }
    }
    unreachable!()
}

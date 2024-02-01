pub fn solution() -> u32 {
    let a: u32 = (1..101).map(|i| i * i).sum();
    let b: u32 = (1..101).sum();
    let b = b * b;
    b - a
}

pub fn solution() -> u32 {
    (0..1000).filter(|i| i % 3 == 0 || i % 5 == 0).sum::<u32>()
}

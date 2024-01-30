pub fn solution() {
    let yep = (0..1000).filter(|i| i % 3 == 0 || i % 5 == 0).sum::<u32>();
    println!("p001: {yep}");
}

pub fn solution() {
    let mut n = 20;
    'main_loop: loop {
        n += 1;
        for i in 1..21 {
            if n % i != 0 {
                continue 'main_loop;
            }
        }
        println!("p005: {n}");
        return;
    }
}

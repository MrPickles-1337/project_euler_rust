pub fn solution() {
    let yep = (100..1000)
        .flat_map(|i| (100..1000).map(move |j| (i, j)))
        .map(|(i, j)| i * j)
        .filter(|i| {
            let s = i.to_string();
            s == s.chars().rev().collect::<String>()
        })
        .max()
        .unwrap();
    println!("p004: {yep}");
}

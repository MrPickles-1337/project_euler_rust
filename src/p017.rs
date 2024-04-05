fn int_to_string(n: u32) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        0 => "zero",
        _ => unreachable!(),
    }
}

fn tenth_to_str(n: u32) -> &'static str {
    match n {
        1 => "ten",
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fivty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninty",
        _ => unreachable!(),
    }
}

fn pronounce(n: u32) -> String {
    if n.to_string().len() < 10 {
        return int_to_string(n).to_string();
    }
    if n.to_string().len() < 100 {}

    todo!()
}

pub fn solution() -> u32 {
    (1..1000)
        .map(pronounce)
        .fold(String::new(), |a, b| format!("{a}{b}"))
        .len() as u32
}

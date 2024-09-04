fn int_to_string(n: u32) -> String {
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
    .to_string()
}

fn tenth_to_str(n: u32) -> String {
    let tmp;
    match n {
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        15 => "fifteen",
        18 => "eighteen",
        14 | 16..=17 | 19 => {
            tmp = format!("{}teen", int_to_string(n % 10));
            tmp.as_str()
        }
        20 => "twenty",
        21..=29 => {
            tmp = format!("twenty{}", int_to_string(n - 20));
            tmp.as_str()
        }
        30 => "thirty",
        31..=39 => {
            tmp = format!("thirty{}", int_to_string(n - 30));
            tmp.as_str()
        }
        40 => "forty",
        41..=49 => {
            tmp = format!("forty{}", int_to_string(n - 40));
            tmp.as_str()
        }
        50 => "fifty",
        51..=59 => {
            tmp = format!("fifty{}", int_to_string(n - 50));
            tmp.as_str()
        }
        60 => "sixty",
        61..=69 => {
            tmp = format!("sixty{}", int_to_string(n - 60));
            tmp.as_str()
        }
        70 => "seventy",
        71..=79 => {
            tmp = format!("seventy{}", int_to_string(n - 70));
            tmp.as_str()
        }
        80 => "eighty",
        81..=89 => {
            tmp = format!("eighty{}", int_to_string(n - 80));
            tmp.as_str()
        }
        90 => "ninety",
        91..=99 => {
            tmp = format!("ninety{}", int_to_string(n - 90));
            tmp.as_str()
        }
        _ => unreachable!(),
    }
    .to_string()
}

fn hundreds_to_str(n: u32) -> String {
    let mut result = format!("{}{}", int_to_string(n / 100), "hundred");
    if n % 100 > 9 {
        result.push_str(format!("and{}", tenth_to_str(n % 100)).as_str());
    } else if n % 10 > 0 {
        result.push_str(format!("and{}", int_to_string(n % 10)).as_str());
    }
    result
}

fn pronounce(n: u32) -> String {
    let n_len = n.ilog10() + 1;
    match n_len {
        1 => int_to_string(n),
        2 => tenth_to_str(n),
        3 => hundreds_to_str(n),
        _ => unreachable!(),
    }
}

pub fn solution() -> u32 {
    // (1..1000).map(pronounce).for_each(|yep| println!("{yep}"));
    (1..1000)
        .map(pronounce)
        .map(|n| n.chars().count() as u32)
        .sum::<u32>()
        + "onethousand".chars().count() as u32
    // todo!()
}

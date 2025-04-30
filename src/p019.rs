use chrono::{Datelike, NaiveDate, Weekday};

pub fn solution() -> u32 {
    let mut count = 0;
    for year in 1901..2001 {
        for month in 1..13 {
            if NaiveDate::from_ymd_opt(year, month, 1).unwrap().weekday() == Weekday::Sun {
                count += 1;
            }
        }
    }
    count
}

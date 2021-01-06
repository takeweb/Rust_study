use std::fs;
// use std::path::PathBuf;
use std::path::Path;
use chrono::{Local, DateTime, Datelike};
use jpholiday::jpholiday::JPHoliday;
use jpholiday::chrono::{NaiveDate};
use std::borrow::Borrow;
use std::convert::TryFrom;
use std::env;

fn main() {
    let year: i32;
    let month: u32;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let local_datetime: DateTime<Local> = Local::now();
        year = local_datetime.year();
        month = local_datetime.month();
    } else {
        year = *(&args[1].parse::<i32>().unwrap());
        month = *(&args[2].parse::<u32>().unwrap());    
    }

    let dir = format!("./test/{}/{:02}", year, month);
    let path = Path::new(&dir);
    let jpholiday = JPHoliday::new();
    let max_day: u32 = TryFrom::try_from(get_days_from_month(year, month)).unwrap();

    for i in 1..=max_day {
        let holiday_flg = jpholiday.is_holiday(NaiveDate::from_ymd(year, month, i).borrow());
        if !holiday_flg {
            let ymd = format!("{}{:02}{:02}", year, month, i);
            let new_path = path.join(ymd);
            fs::create_dir_all(new_path)
                .expect("error");
        }
    }
}

fn get_days_from_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days()
}

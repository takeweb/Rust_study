use std::fs;
use std::path::Path;
use chrono::{Local, DateTime, Datelike};
use std::env;
use my_lib::date_util;

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
    let mut my_holiday = date_util::MyHoliday::new();
    let max_day: u32 = date_util::get_days_from_ym(year, month);

    for i in 1..=max_day {
        if my_holiday.is_bizday(year, month, i) {
            let ymd = format!("{}{:02}{:02}", year, month, i);
            let new_path = path.join(ymd);
            fs::create_dir_all(new_path)
                .expect("error");
        }
    }
}

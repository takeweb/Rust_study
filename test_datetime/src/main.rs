use chrono::{Local, DateTime, Date, Datelike, Duration, NaiveDateTime, NaiveDate};
use jpholiday::jpholiday::JPHoliday;
// use jpholiday::chrono::{NaiveDate};
use std::borrow::Borrow;

fn main() {
    let local_datetime: DateTime<Local> = Local::now();
    println!("{}", local_datetime);

    let local_date: Date<Local> = Local::today();
    println!("{}", local_date);

    let local_datetime_str = Local::now().format("%Y年%m月%d日 %H時%M分%S秒").to_string();
    println!("{}", local_datetime_str);

    let year = local_datetime.year();
    let month = local_datetime.month();
    let day = local_datetime.day();
    let ymd: String = format!("{}{:02}{:02}", year, month, day);
    println!("{}", ymd);

    let jpholiday = JPHoliday::new();
    let holiday_flg = jpholiday.is_holiday(NaiveDate::from_ymd(2017, 1, 1).borrow());
    println!("{}", holiday_flg);
    
    let tomorrow = local_datetime + Duration::days(1);
    println!("{}", tomorrow);
    
    let yesterday = local_datetime - Duration::days(1);
    println!("{}", yesterday);

    // let before_month = local_datetime - Duration::month(1);
    // println!("{}", before_month);

    // Timezoneあり
    let dt = DateTime::parse_from_str("2018/12/07 19:31:28 +09:00", "%Y/%m/%d %H:%M:%S %z").unwrap();
    println!("DateTime::parse_from_str: {:?}", dt);

    // Timezoneなし
    let dt = NaiveDateTime::parse_from_str("2018/12/07 19:31:28", "%Y/%m/%d %H:%M:%S").unwrap();
    println!("NaiveDateTime::parse_from_str: {:?}", dt);

    // 日付だけならTimezoneが不要な場合
    let date = NaiveDate::from_ymd(2015, 3, 14);
    println!("{}", date.year());
    println!("{}", date.month());
    println!("{}", date.day());

    let days = get_days_from_month(2020, 2);
    println!("2020年02月は{}日", days);
}

pub fn get_days_from_month(year: i32, month: u32) -> i64 {
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


#[test]
fn test_is_holiday() {
    let jpholiday = JPHoliday::new();
    assert!(jpholiday.is_holiday(NaiveDate::from_ymd(2021, 1,  1).borrow()) == true);
    assert_eq!(jpholiday.is_holiday(NaiveDate::from_ymd(2021, 1,  2).borrow()), false);
    assert_eq!(jpholiday.is_holiday(NaiveDate::from_ymd(2021, 1,  2).borrow()), false);
    assert_eq!(jpholiday.is_holiday(NaiveDate::from_ymd(2021, 2, 11).borrow()), true);
    assert_eq!(jpholiday.is_holiday(NaiveDate::from_ymd(2021, 2, 23).borrow()), true);
}

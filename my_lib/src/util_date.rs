use std::borrow::Borrow;
use std::convert::TryFrom;
use chrono::{NaiveDate};
use jpholiday::jpholiday::JPHoliday;
use super::util_file;

pub struct MyHoliday<'a> {
    jpholiday: JPHoliday<'a>,
    myreg: Vec<NaiveDate>,
}

impl<'a> MyHoliday<'a> {
    pub fn new(hol_path: &str) -> Self {
        let datas = util_file::get_contents(hol_path).ok().unwrap();
        let mut naive_dates = Vec::new();
        for data in datas {
            let v: Vec<&str> = data.split('-').collect();
            naive_dates.push(NaiveDate::from_ymd(*&v[0].parse().unwrap(), *&v[1].parse().unwrap(), *&v[2].parse().unwrap()));
        }
        
        MyHoliday {
            jpholiday: JPHoliday::new(),
            myreg: naive_dates
        }
    }
    pub fn is_bizday(&mut self, year: i32, month: u32, day: u32) -> bool {
        let mut bizday_flg = !self.jpholiday.is_holiday(NaiveDate::from_ymd(year, month, day).borrow());
        for holiday in &self.myreg {
            if *holiday == NaiveDate::from_ymd(year, month, day) {
                bizday_flg = false;
            }
        }
        bizday_flg
    }
}

pub fn get_days_from_ym(year: i32, month: u32) -> u32 {
    TryFrom::try_from(NaiveDate::from_ymd(
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
    .num_days()).unwrap()
}

pub fn is_leap(year: u32) -> bool {
    let rtn: bool;
    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        rtn = true;            
    } else {
        rtn = false;
    }
    rtn
}

#[test]
fn test_get_days_from_ym() {
    assert_eq!(get_days_from_ym(2021, 1), 31);
    assert_eq!(get_days_from_ym(2021, 2), 28);
    assert_eq!(get_days_from_ym(2020, 2), 29);
    assert_eq!(get_days_from_ym(2021, 3), 31);
}

#[test]
fn test_is_leap() {
    assert!(is_leap(2000) == true);
    assert!(is_leap(2100) == false);
    assert!(is_leap(2019) == false);
    assert!(is_leap(2020) == true);
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
    // let datas = get_contents().ok().unwrap();
    // for data in datas {
    //     // println!("{}", data);
    //     let v: Vec<&str> = data.split('-').collect();
    //     println!("{}", format!("{}{:02}{:02}", &v[0], &v[1], &v[2]));
    // }
}

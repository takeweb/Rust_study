use std::fs;
use std::path::Path;
use chrono::{Local, DateTime, Datelike};
use std::env;
use my_lib::util_date;
use dotenv::dotenv;

fn main() {
    let year: i32;
    let month: u32;

    // 引数取得
    let args: Vec<String> = env::args().collect();

    // 引数指定がない場合、現在日付を基準とする
    if args.len() < 2 {
        let local_datetime: DateTime<Local> = Local::now();
        year = local_datetime.year();
        month = local_datetime.month();
    } else {
        year = *(&args[1].parse::<i32>().unwrap());
        month = *(&args[2].parse::<u32>().unwrap());    
    }

    // .envファイルから設定を取得
    dotenv().ok();
    let out_path_conf = env::var("OUT_PATH")
                           .expect("OUT_PATH is not found");
    let out_path_list: Vec<&str> = out_path_conf
                                    .split(',')
                                    .collect();
    let hol_path = env::var("HOL_PATH")
                            .expect("HOL_PATH is not found");

    // 日付ユーティリティから休日をインスタンス化
    let mut my_holiday = util_date::MyHoliday::new(&hol_path);
    let max_day = util_date::get_days_from_ym(year, month);

    // 出力先ディレクトリ分繰り返し
    for out_path in out_path_list {
        let dir = format!("{}/{}/{:02}", out_path, year, month);
        let path = Path::new(&dir);    

        for day in 1..=max_day {
            if my_holiday.is_bizday(year, month, day) {
                let ymd = format!("{}{:02}{:02}", year, month, day);
                let new_path = path.join(ymd);
                fs::create_dir_all(new_path)
                    .expect("error");
            }
        }
    }
}

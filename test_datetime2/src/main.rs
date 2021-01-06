use chrono::NaiveDate;

fn main() {
    let year = 2020;
    for (m, d) in (1..=12).map(|m| {
        (
            m,
            if m == 12 {
                NaiveDate::from_ymd(year + 1, 1, 1)
            } else {
                NaiveDate::from_ymd(year, m + 1, 1)
            }.signed_duration_since(NaiveDate::from_ymd(year, m, 1))
            .num_days(),
        )
    }) {
        println!("days {} in month {}", d, m);
    }
}
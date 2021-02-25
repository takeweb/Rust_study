use clap::{App, Arg};
use std::error::Error;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use hdrhistogram::Histogram;

type LocId = u16;
type DT = NaiveDateTime;
type AppResult<T> = Result<T, Box<dyn Error>>;

fn parse_datetime(s: &str) -> AppResult<DT> {
    DT::parse_from_str(s, "%Y-%m-%d %H:%M:%S").map_err(|e| e.into())
}

#[derive(Debug, Deserialize)]
struct Trip {
    #[serde(rename = "tpep_pickup_datetime")]
    pickup_datetime: String,    // 乗車日時

    #[serde(rename = "tpep_dropoff_datetime")]
    dropoff_datetime: String,   // 降車日時

    #[serde(rename = "PULocationID")]
    pickup_loc: LocId,          // 乗車地ID

    #[serde(rename = "DOLocationID")]
    dropoff_loc: LocId,         // 降車地ID
}

#[derive(Debug, Serialize)]
struct RecordCounts {
    read: u32,      // CSVファイルから読み込んだそうレコード数
    matched: u32,   // 乗車地や降車地などの条件を満たしたレコード数
    skipped: u32,   // 条件は満たしたが異常値により除外したレコード数
}

impl Default for RecordCounts {
    fn default() -> Self {
        Self {
            read: 0,
            matched: 0,
            skipped: 0,    
        }
    }
}

struct DurationHistograms(Vec<Histogram<u64>>);
impl DurationHistograms {
    fn new() -> AppResult<Self> {
        let lower_bound = 1;            // 記録する下限値：1秒
        let upper_bound = 3 * 60 * 60;  // 記録する上限値：3時間
        let hist = Histogram::new_with_bounds(lower_bound, upper_bound, 3)
            .map_err(|e| format!("{:?}", e))?;
        let histograms = std::iter::repeat(hist).take(24).collect();
        Ok(Self(histograms))
    }
    fn record_duration(&mut self, pickup: DT, dropoff: DT) -> AppResult<()> {
        let duration = (dropoff - pickup).num_seconds() as u64;
        // 20分未満
        if duration < 20 * 60 {
            Err(format!("duration secs {} is too short.", duration).into())
        } else {
            let hour = pickup.hour() as usize;
            self.0[hour]
                .record(duration)
                .map_err(|e| {
                    format!("duration secs {} is too long. {:?}", duration, e)
                        .into()
                })
        }
    }
}

fn analyze(infile: &str) -> AppResult<String> {
    let mut reader = csv::Reader::from_path(infile)?;
    let mut rec_counts = RecordCounts::default();
    let mut hist = DurationHistograms::new()?;
    for (i, result) in reader.deserialize().enumerate() {
        let trip: Trip = result?;
        rec_counts.read += 1;
        if is_jfk_airport(trip.dropoff_loc) && is_in_midtown(trip.pickup_loc) {
            let pickup = parse_datetime(&trip.pickup_datetime)?;
            if is_weekday(pickup) {
                rec_counts.matched += 1;
                let dropoff = parse_datetime(&trip.dropoff_datetime)?;
                hist.record_duration(pickup, dropoff)
                    .unwrap_or_else(|e| {
                        eprintln!("WARN: {} - {}. Skipped: {:?}", i + 2, e, trip);
                        rec_counts.skipped += 1;
                    });
            }
        }
        if rec_counts.read <= 10 {
            println!("{:?}", trip);
        }
    }
    println!("{:?}", rec_counts);
    let display_stats = DisplayStats::new(rec_counts, hist);
    let json = serde_json::to_string_pretty(&display_stats)?;
    Ok(json)
}

fn is_in_midtown(loc: LocId) -> bool {
    let locations = [90, 100, 161, 162, 163, 164, 186, 230, 234];
    locations.binary_search(&loc).is_ok()
}

fn is_jfk_airport(loc: LocId) -> bool {
    loc == 132
}

fn is_weekday(datetime: DT) -> bool {
    datetime.weekday().number_from_monday() <= 5
}

fn main() {
    let arg_matches = App::new("trip-analyzer")
        .version("1.0")
        .about("Analyze yellow cab trip records")
        .arg(Arg::with_name("INFILE")
            .help("Sets the input CSV file")
            .index(1)
            .required(true)
        )
        .get_matches();
    let infile = arg_matches.value_of("INFILE").unwrap();
    println!("INFILE: {:?}", infile);
    match analyze(infile) {
        Ok(json) => println!("{}", json),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[derive(Serialize)]
struct DisplayStats {
    rec_counts: RecordCounts,
    stats: Vec<StatsEntry>,
}

#[derive(Serialize)]
struct StatsEntry {
    hour_of_day: u8,    // 0〜23。時(hour)を表す
    minimum: f64,       // 最短の所要時間
    median: f64,        // 所要時間の中央値
    #[serde(rename = "95th percentile")]
    p95: f64,
}
impl DisplayStats {
    fn new(rec_counts: RecordCounts, histograms: DurationHistograms) -> Self {
        let stats = histograms.0.iter().enumerate()
            .map(|(i, hist)| StatsEntry {
                hour_of_day: i as u8,
                minimum: hist.min() as f64 / 60.0,
                median: hist.value_at_quantile(0.5) as f64 / 60.0,
                p95: hist.value_at_quantile(0.95) as f64 / 60.0,
            })
            .collect();
        Self {
            rec_counts,
            stats,
        }
    }
}
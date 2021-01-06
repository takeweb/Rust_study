use rusqlite::{params, Connection, Result};
// use chrono::prelude::*;

#[derive(Debug)]
struct Health {
    data_id: i32,
    // regist_date: NaiveDate,
    regist_date: String,
    height: f64,
    weight: f64,
    bmi: f64,
}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let cn = Connection::open("healthcare.db")?;

    if args.len() <= 1 {
        // パラメータ指定がない場合
        let mut stmt = cn.prepare(r#"
            SELECT
                  data_id
                , regist_datetime
                , height
                , weight
                , bmi
            FROM health
            ORDER BY
            regist_datetime"#)?;
        for it in stmt.query_map(params![], |row| {
            Ok(Health{
                data_id: row.get(0)?,
                //regist_date: NaiveDate::parse_from_str(&dt, "%Y-%m-%d").unwrap(),
                regist_date: row.get(1)?,
                height: row.get(2)?,
                weight: row.get(3)?,
                bmi: row.get(4)?,
            })
        })? {
            println!("{:?}", it.unwrap());
        }

    } else {
        match args[1].as_str() {
            "init" => {
                cn.execute(r#"
                    CREATE TABLE IF NOT EXISTS health(
                          data_id INTEGER PRIMARY KEY AUTOINCREMENT
                        , regist_datetime TEXT
                        , height FLOAT
                        , weight FLOAT
                        , bmi FLOAT
                    )
                    "#, params![])?;
                println!("create database.");    
            },
            "into" => {
                // データ挿入
                let regist_date = &args[2];
                let height: f64 = args[3].parse::<f64>().unwrap();
                let weight: f64 = args[4].parse::<f64>().unwrap();
                let bmi: f64 = args[5].parse::<f64>().unwrap();
                let mut stmt = cn.prepare(r#"
                    INSERT INTO health(
                        regist_datetime
                        , height
                        , weight
                        , bmi
                    )
                    VALUES(?, ?, ?, ?)"#)?;
                stmt.execute(params![regist_date, height, weight, bmi])?;
            },
            _ => {
                println!("parameter error.");
            }
        }
    }
    Ok(())
}
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_contents(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let mut datas = Vec::new();
    for line in BufReader::new(file).lines() {
        datas.push(line?.to_string());
    }
    Ok(datas)
}
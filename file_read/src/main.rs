use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let datas = get_contents().ok().unwrap();
    for data in datas {
        let v: Vec<&str> = data.split('-').collect();
        println!("{}", format!("{}{:02}{:02}", &v[0], &v[1], &v[2]));
    }
}

pub fn get_contents() -> std::io::Result<Vec<String>> {
    let path = "sample.txt";
    let file = File::open(path)?;
    let mut datas = Vec::new();
    for line in BufReader::new(file).lines() {
        datas.push(line?.to_string());
    }
    Ok(datas)
}

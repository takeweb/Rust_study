use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let cn = Connection::open("sample.db")?;

    if args.len() <= 1 {
        // パラメータ指定がない場合
        let mut stmt = cn.prepare("SELECT * FROM users ORDER BY id")?;
        for it in stmt.query_map(params![], |row| {
            Ok(User{
                id: row.get(0)?,
                name: row.get(1)?,
                age: row.get(2)?,
            })
        })? {
            println!("{:?}", it.unwrap());
        }

    } else {
        match args[1].as_str() {
            "init" => {
                cn.execute("CREATE TABLE users(id INTEGER, name TEXT, age INTEGER)", params![])?;
                println!("create database.");    
            },
            "into" => {
                // データ挿入
                let id: i32 = args[2].parse::<i32>().unwrap();
                let name = &args[3];
                let age: i32 = args[4].parse::<i32>().unwrap();
                
                let mut stmt = cn.prepare("INSERT INTO users VALUES(?, ?, ?)")?;
                stmt.execute(params![id, name, age])?;
            },
            _ => {
                println!("parameter error.");
            }
        }
    }
    Ok(())
}
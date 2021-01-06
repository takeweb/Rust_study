use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

fn main() -> Result<()>{
    // インメモリデータベースの作成
    let cn = Connection::open_in_memory()?;

    // テーブルを作成
    cn.execute("CREATE TABLE users(id INTEGER, name TEXT, age INTEGER)", params![])?;
    let mut stmt = cn.prepare("INSERT INTO users(id, name, age) VALUES(?, ?, ?)")?;
    stmt.execute(params![1, "Kongo", 20])?;
    stmt.execute(params![2, "Goto", 21])?;
    stmt.execute(params![3, "Nabeshima", 30])?;
    stmt.execute(params![4, "Kirishima", 15])?;

    // クエリの作成
    let mut stmt = cn.prepare("SELECT * FROM users WHERE age > ?")?;
    let iter = stmt.query_map(params![15], |row| {
        Ok(User{
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;
    
    for it in iter {
        println!("{:?}", it.unwrap());
    }

    Ok(())
}

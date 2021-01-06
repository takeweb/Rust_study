use rusqlite::{params, Connection, Result};

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
    let mut stmt = cn.prepare("SELECT * FROM users")?;
    let mut rows = stmt.query(params![])?;
    while let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        let age: i32 = row.get(2)?;
        println!("id; {}, name: {}, age: {}", id, name, age);
    }

    Ok(())
}

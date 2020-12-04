use::std::io;

fn main() {
    // 標準入力から段数を取得して実行
    let mut n = String::new();
    //io::stdin().read_line(&mut n).ok();
    io::stdin().read_line(&mut n)
        .expect("Failed to read line");

    hanoi(n.trim().parse().ok().unwrap(), 'a', 'b', 'c');
    // let input: i32 = match n.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => 
    // };
    //hanoi(input, 'a', 'b', 'c');
}

fn hanoi(n: i32, from: char, to: char, via: char) {
    if n > 1 {
        hanoi(n - 1, from, via, to);
        println!("{} -> {}", from, to);
        hanoi(n - 1, via, to, from);
    } else {
        println!("{} -> {}", from, to);
    }
}


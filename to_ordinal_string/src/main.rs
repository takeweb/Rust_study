fn main() {
    for i in 1..=10 {
        println!("Hello, {} world!", to_ordinal_string(i));
    }
}

fn to_ordinal_string(n: usize) -> String {
    let s = match n % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    // format!("{0: <02}{1}", n, s)
    format!("{:<02}{}", n, s)
}
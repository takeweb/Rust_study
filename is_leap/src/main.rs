use std::io;

fn main(){
    let mut year = String::new();
    
    io::stdin().read_line(&mut year)
        .expect("Failed to read line");
    
    let year: u32 = match year.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if is_leap(year) {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn is_leap(year: u32) -> bool {
    let rtn: bool;
    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        rtn = true;            
    } else {
        rtn = false;
    }
    rtn
}

#[test]
fn test_is_leap() {
    assert!(is_leap(2000) == true);
    assert!(is_leap(2100) == false);
    assert!(is_leap(2019) == false);
    assert!(is_leap(2020) == true);
}
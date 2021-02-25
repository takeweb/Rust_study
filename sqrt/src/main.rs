fn main() {
    let a = 4.0;
    println!("sqrt({}) = {}", a, sqrt(a));
}

fn sqrt(a: f64) -> f64 {
    let mut x0 = if a > 1.0 {
        a
    } else {
        1.0
    };

    loop {
        let x1 = (x0 + a / x0) / 2.0;
        if x1 >= x0 {
            break;
        }
        x0 = x1;
    }
    x0
}

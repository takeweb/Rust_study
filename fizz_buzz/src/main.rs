fn main() {
    for i in 1..=100 {
        match i {
            e if e % 15 == 0 => println!("FizzBuzz"),
            e if e %  3 == 0 => println!("Fizz"),
            e if e %  5 == 0 => println!("Buzz"),
            _e => println!("{}", _e)
        }
    }
}

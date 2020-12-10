
#[derive(Debug)]
struct Reactangle {
    width: u32,
    height: u32,
}

impl Reactangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }    
}

fn main() {
    let rect1 = Reactangle { width: 30, height: 50};
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of rectangle is {} square pixels."
        , rect1.area()
    );
}

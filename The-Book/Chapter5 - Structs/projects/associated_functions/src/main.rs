#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 3,
        height: 5
    };
    let sq = Rectangle::square(5);
    if sq.width() {
        println!("The Rectangle has a width {}", sq.width);
    }
    println!("Area of rect1 is {}", rect1.area());
    println!("{:#?}",sq);
}

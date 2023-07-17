#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };
    println!("{:?}",rect); // prints in same line
    println!("{:#?}",rect); // formats properly
}

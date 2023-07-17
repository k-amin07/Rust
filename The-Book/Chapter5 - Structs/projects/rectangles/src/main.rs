struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels", area_naive(width1,height1));
    // the above method of finding area works fine, but nowhere does the code indicate 
    // that width and height are related variables. We can refactor the code with tuples
    // as follows

    let rect1 = (30,50);
    println!("The area of the rectangle is {} square pixels", area_tuples(rect1));

    // Tuples dont name their elemements so while the code is a bit more structured, we dont know
    // whether the first element is the width or the height, making the code less clear. We can
    // refactor it with structs as follows

    let rect2 = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {} square pixels", area(&rect2));
    // println!("{}",rect2); // throws error that `Rectangle` cannot be formatted with the default formatter
    // the reason being that `Rectangle` doesn't implement `std::fmt::Display` 
}

fn area_naive(width:u32,height:u32) -> u32 {
    width * height
}

fn area_tuples(dimensions:(u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(rectangle: &Rectangle) -> u32 { // area function borrows the struct, the ownership remains with caller
    rectangle.width * rectangle.height
}


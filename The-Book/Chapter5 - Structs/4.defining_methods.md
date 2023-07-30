We can define methods on structs using the `impl` keyword. Methods have the same signature as functions and work in the same way except that they are defined on structs and the first parameter is the reference to the calling instance. In the Rectangle example, we can define the `area` method on a rectangle as follows

```
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) {
        self.width * self.height
    }
}
```

Here we have access to the `&self` reference, just like in python. Everything in this `impl` block will be associated to the rectangle type. `&self` should be the first parameter to the functions in the block. `&self` is actually short for `self:&self`. In the `impl` block, `self` is an alias for the type the `impl` block is for.

In main, we can use the method syntax (like `rect1.area()`) on instances of rectangle. We use `&self` because we do not want to take ownership. We just want to read data and not write to it. A method can also have the same name as one of the fields. For example,

```
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    // method with multiple parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 2,
        height: 4
    };
    if rect1.width() { 
        // note that rect1.width will break since that is u32. rect1.width() returns a boolean so that is fine
        println!("Rectangle has non zero width {}",rect.width);
    }
}
```
This syntax, where a method is given the same name as the field, is often used to implement getters. Rust does not automatically implement them for struct fields. Getters are useful as we can make a field private but the method public. There is no `->` operator in Rust. In C++, `.` is used for calling a method on an object directly while `->` is used for calling a method on a pointer to an object. In Rust, the following lines are equivalent and Rust uses a feature here called "automatic referencing and dereferencing.
```
p1.distance(&p2);
(&p1).distance(&p2);
```

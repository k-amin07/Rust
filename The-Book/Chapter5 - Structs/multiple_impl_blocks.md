Each struct is allowed to have multiple `impl` blocks. There is no reason to have them separate, but the syntax is valid. These are useful in some cases that are discussed in Chapter 10.

```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn width(&self) -> {
        self.width
    }
}
```
All functions in an `impl` block are associated functions as they are associated with the type name after the `impl` keyword. We can define associated functions that are not methods (do not have self as the first parameter) as they do not need an instance of the said type to work with. One such example is the `String::from()` method.

Associated funcitions that arent methods are often used for constructors that will return a new instance of the struct. These are often called `new` but `new` is not a special name and isnt built into the language. For example, we can use the rectangle struct to create squares like

```
impl Rectangle {
    fn square(size:u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}
```
Here the function returns a value of type `Self` which is an alias for the typle being `impl`emented. To call this associated method, we'd have to use the `::` syntax with the Struct name, e.g. `let sq = Rectangle::square(3);`
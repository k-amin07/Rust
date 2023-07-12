- Defined using the fn keyword.
- Arguments have specified datatypes
- Function may optionally have a return type if it returns something.
- If the last statement of a function sequence is a variable or constant, it is treated as return statement even if there is no explicit return statement. For example
```
    fn function_no_return() -> i32 {
        2
    }
```
is valid, and so is
```
    if true {
        5
    } else {
        2
    }
```
However, the following isnt
```
    if true {
        5
    }
    2
```
Better to stay away from this shorthand imo, and just use explicit return statements.
A function with no return type cannot be used in a print statement (will cause a compile time error).

This code will also produce an error
```
    fn plus_one(x: i32) -> i32 {
        x + 1;
    }
```
(refer to [expressions](./expressions.md)) for details.
### if-expressions
If expressions in Rust require a boolean value. An integer or string will not evaluate to true or false, but will create a compile time error instead.
```
    if true {
        ...
    }
```
is a valid if statement, 
```
    if 2 {
        ...
    }
```
is not. If conditions can also be used in assignment, for example,
```
    let condition = true
    let x = if condition {
        3
    } else {
        4
    }
```
will set x to 3. However, the types need to be compatible. The following will raise an error
```
    let x = if condtion {
        3
    } else {
        "four"
    }
```

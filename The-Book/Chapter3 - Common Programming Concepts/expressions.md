Expressions evaluate to something and make up most of the code in rust.
```
    let y = 3;
```
is an expression.

```
    let y = {
        let x = 3;
        x * x * x
    };
```
is also a valid expression which stores 27 in y. {} block creates a new scope which is also an expression. If a semicolon is added to the line `x*x*x`, it becomes a statement and will not return anything.
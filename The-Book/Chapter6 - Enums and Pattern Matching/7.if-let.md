`if let` syntax combines `if` and `let` statements into a less verbose way of handling a value that matches a pattern while ignoring the rest. In match, for an `integer`, we would have to define `Some` and `None` or `_`. For example

```
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The max is {}",max),
    _ => ()
}
```
Here we match on a `Option<u8>` value but only want to execute code if it is a `Some` variant. This can be done more concisely with the if let statement

```
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum number is configured to be {}",max);
}
```

The `if let` syntax takes a pattern and an expression separated by a `=` sign (not a `==`). It works the same way as a `match`, where the expression is given to the `match` and the pattern is its first arm. In this case, the pattern is `Some(max)` and max binds to the varible inside `Some`. We can use this `max` in the body of the `if let` just like we did with match.

`if let` syntax requires less boilerplate code but we lose exhaustive matching. The choice between `if let` and match often depends on the problem at hand. To implemet `_` with `if let`, we can introduce the `else` clause.

In our coin example, if we want to count all the non quarter coins and announce everytime we see a quarter, the code will be as follows

- Using `match`
```
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State Quarter from {:#?}!",state),
    _ => count += 1,
}
```

- Using `if let`
```
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State Quarter from {:#?}!",state);
} else {
    count += 1;
}
```
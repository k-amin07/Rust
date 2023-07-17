`match` is an extremely powerful control flow construct in Rust. It allows us to compare a value against a series of patterns and then execute code based on which pattern matches. Patterns can be made up of literal values, variable names, and many other things, covered in detail in Chapter 18.

`match` is like a coin sorting machine. A coin falls through the first hole that it fits into. In the same way, values go through each pattern in match and the code block associated to the first pattern that matches executes. Refer to [match_control_flow](./projects/match_control_flow/src/main.rs) for code. This is kind of similar to `switch-case`.

The main difference between `match` and `if` in Rust is that `if` requires a boolean value while `match` can work with any type.

A `match` pattern does not have to be a single line of code, a code block can be added usign `{}`.
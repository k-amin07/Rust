Contains important points to remember from the chapter-wise notes for quick lookup

- `cargo new <project_name> --bin` creates an executable application.
- `cargo run` runs the built application
- `cargo build --release` creates a release build.
- `cargo doc --open` will generate documentation for the project (including dependencies) and open it in the browser.

- A function accepting a string as an argument should accept `&str` (string literal) instead of `&String` because string literals are immutable slices. They will work with both string literals and `String`s.
- Refer to [Chapter4](./The-Book/Chapter4%20-%20Understanding%20Ownership/slices.md) for Strings.


- Structs can hold different types of data
    - `impl structName` is used to define methods on structs
    - A struct can have multiple `impl` blocks which can be useful in some situations.
    - A method implemented on a struct can return an instance of the struct (like a constructor) by returning `Self { }`.

- Common collections include vectors, strings, hashmaps and binary heaps
- Vectors can be initialized with Vec::new() or vec! macro
- Strings are UTF-8 encoded. str is a string literal
    - Strings support push for one character and push_str for appending a string slice
    - Any type can be converted to string as long as it implements the Display trait.
    - `format!` macro is better to concat strings than `+` operator because it does not take ownership of the string being concatenated.
    - We can iterate over string using `for c in mystring.chars()` or `for c in mystring.bytes()` (to get raw bytes).
    - We can also obtain a slice of the string instead by using `let s = &hello[0..4];`


    
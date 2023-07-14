Contains important points to remember from the chapter-wise notes for quick lookup

- `cargo new <project_name> --bin` creates an executable application.
- `cargo run` runs the built application
- `cargo build --release` creates a release build.
- `cargo doc --open` will generate documentation for the project (including dependencies) and open it in the browser.

- A function accepting a string as an argument should accept `&str` (string literal) instead of `&String` because string literals are immutable slices. They will work with both string literals and `String`s.
- Refer to [Chapter4](./The-Book/Chapter4%20-%20Understanding%20Ownership/slices.md) for Strings.
In the previous example, if we try to print a variable of type Rectangle, the compiler throws the following error
```
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```
The println! macro can do many kinds of formatting, and by default, the curly brackets tell println! to use formatting known as `Display:` output intended for direct end user consumption. The primitive types implement `Display` by default. With structs, Rust does not try to guess how we'd want to format the output since there are many possibilities like should it display commas or not, should it print curly braces or not etc. The compiler provides the following help in this case

```
   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```
Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. This can be done by adding `#[derive(Debug)]` and printing using `{:?}`
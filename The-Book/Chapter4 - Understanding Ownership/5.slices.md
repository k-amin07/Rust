The idea for slices is similar to array/string slices in other languages. Basically a contiguous sequence of elements. The book discusses an interesting usecase, where if you had a program that returns length of the first word found in an input string (refer to `first_word` function in [projects/slices](./projects/slices/src/main.rs)). In the main function, we get the length of the first word by calling the `first_word` function on a string `s`, but clear the string afterwards. Now the length (stored in the variable `word`) is meaningless since it was valid for that particular particular string and the string has now been cleared.

Having to worry about the index in word getting out of sync with the data in `s` is tedious and error prone. If we change the function to return the start and end index, we have even more values that were calculated from data in a particular state but aren’t tied to that state at all. In this case we'd have three variables that we'd have to worry about keeping in sync. Rust has a solution for this, i.e. String slices

### String slices
A string slice is a reference to a part of the String and looks like this
```
let s = String::from("Hello World");
let hello = &s[0..5];
let world = &s[6..11];
```
Similar to python, the first index is included, the second index is not. Variables `hello` and `world` reference to a portion of the String. Internally, the slice data structure stores a pointer to the starting position and length of the slice. If the slice starts at 0th index, the first number can be be omitted. Likewise, if the slice ends at the last element, the second number can be omitted. Likewise, both numbers can be omitted if the slice contains the entire String.
```
let hello = &s[..5];
let world = &s[6..];
let hello_world = &s[..];
```

Refer to the `first_word_slice` function in [projects/slices](./projects/slices/src/main.rs). In the main function, we store the slice returned by the `first_word_slice` function in variable `word2`. If we clear the string `s` before printing `word2`, we will get a compile time error!

**String Literals** are also slices. The type of string literal is `&str`
```
fn first_word(s: &str) -> &str {
```
will accept both `&String` and `&str` values. If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the String or a reference to the String. This flexibility takes advantage of _deref coercions_ (covered in Chapter 15).

### Other Slices
Array slices exist as well
```
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3]);
```
This slice has the type `&[i32]`. It works in the same way as string slices do. Slices can be used on all sorts of other collections and will be discussed in detail in Chapter 8.
The problem with the code at the end of the last section is having to return the String from the function in order to be able to use it in the current scope. This is not ideal. Luckily, Rust allows passing a reference of the object to avoid this issue.

```
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```
The ampersand allows us to create a reference to the String. The function calculate_length references the String but does not own it. Since calculate_length does not have ownership of the String, the String is not destroyed from memory when the function returns.

Having references as a function parameter is knows as borrowing. This is because the function has borrowed the value and does not own it. However, if the function borrowing the variable tries to change it, Rust throws an error.

```
error: cannot borrow immutable borrowed content `*some_
string` as mutable
--> error.rs:8:5
|
8 | some_string.push_str(", world");
| ^^^^^^^^^^^
```
Just like variables, variables, references are also immutable by default. This can be fixed by defining the reference as mutable. 

```
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
Mutable references have one big restriction. Only one mutable reference to a particular variable can exist in a given scope
```
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;
```
will generate an error
```
error[E0499]: cannot borrow `s` as mutable more than once
at a time
--> borrow_twice.rs:5:19
|
4 | let r1 = &mut s;
| - first mutable borrow occurs here
5 | let r2 = &mut s;
| ^ second mutable borrow occurs here
6 | }
| - first borrow ends here
```
This restriction prevents race conditions (particularly data races) at compile time. A data race is a condition where these three behaviors occur:
- Two or more pointers have access to the same data
- At least one of the pointers is being used to write data
- There is no mechanism being used to synchronize access to that data

The following however is allowed because the references are in different scopes
```
let mut s = String::from("hello");
{
    let r1 = &mut s;
} // r1 goes out of scope here, hence we get no error with r2
let r2 = &mut s;
```
Similar rule exists for combining mutable and immutable references
```
let mut s = String::from("hello");
let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // compiler throws an error cannot borrow `s` as mutable because it is also borrowed as immutable
``` 
### Dangling References
Rust guarantees that there will be no dangling references. The following code will generate a compile time error `missing lifetime specifier` (lifetimes covered in Chapter 10).
```
fn main() {
    let reference_to_nothing = dangle();
}
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
```

The error occurs because s goes out of scope when the function dangle() returns. Since we are returning the reference to a variable that no longer exists, Rust throws an error. The solution is to return the String directly
```
fn no_dangle() -> &String {
    let s = String::from("hello");
    s
}
```

### Rules of reference recap
- At any time, we can have _either_ but not _both_ of the following
    - One mutable reference
    - Any number of immutable references
- References must always be valid.
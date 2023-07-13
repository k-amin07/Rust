### Ownership and Functions
The semantics for passing a variable to a function are similar to assigning a value to a variable. The following code demonstrates variables going in and out of scope.
```
fn main() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // value of s moves into the function
    ...                             // s no longer valid here
    let x = 5;                      // x comes into scope, x is i32
    make_copy(x);                   // i32, hence gets copied into the function
    ...                             // x still valid here.
}
// x goes out of scope. Since s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}",some_string);           // some_string is valid here
}
// some_string goes out of scope, drop is called and the memory is freed.

fn makes_copy(some_integer: i32) {      // some_integer comes into scope
    println!("{}",some_integer);
}
// some_integer goes out of scope, nothing special happens.
```
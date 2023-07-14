### Return Values and Scope
Returning a value also transfers ownership
```
fn main() {
    let s1 = gives_ownership();          // return value of gives_ownership is moved into s1.
    let s2 = String::from("abc");        // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 moves into takes_and_gives_back, its return value moves into s3.
}
// s3 goes out of scope and is dropped. s2 goes out of scope but was moved so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    let some_string = String::from("Hello"); // some_string comes into scope
    some_string                             // some_string moves out to the calling function.
}

fn takes_and_gives_back(a_string:String) -> String {    // a_string comes into scope
    a_string                                            // a_string is returned and moves out to the calling function.
}
```

Multiple values can be returned form a function using tuples
```
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
}

fn calculate_length(s:String) -> (String,i32) {
    let length = s.len();
    (s,length)
}
```
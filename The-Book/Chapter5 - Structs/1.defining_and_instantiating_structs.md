Structs are similar to dicts in python or types in graphql. In Rust itself, they are similar to [tuples](../Chapter3%20-%20Common%20Programming%20Concepts/variables_and_data_types.md). To define a struct, we use the keyword `struct` as follows

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
```

After defining a struct, we can create instances of the struct by providing values for the defined keys
```
fn main() {
    let email = String::from("someuser1@example.com")
    let username =  String::from("someusername1")
    let mut user1 = User {
        active: true,
        username,
        email: email,
        sign_in_count: 0
    };
    user1.sign_in_count = 1;
}
```
The `Dot (.)` operator can be used to get or set values, however, the entire struct instance must be mutable. We cannot set individual fields as mutable. Values can be assigned directly through variables (useful for setting values from function arguments, for example). If the variable name is the same as the struct field name, we can use the _field init shorthand_ syntax (like we did for username in the example above, can also be done for email).

To set a particular value but use rest of the values from a different instance, _struct update syntax_ (`..`) can be used. This is similar to javascript except that the updated fields are defined first and we use two dots instead of three.

```
    let user2 = User {
        email: String::from("someuser2@example.com"),
        ..user1
    }
```
`..user1` must come last. In this case however, the `username` of `user2` borrows from `user1` and hence `user1` can no longer be used. If all _used_ fields of `user1` are copy fields, `user1` would still be valid.

### Tuple Structs
Rust also supports Structs that look similar to tuples, called **tuple structs**. In this case, we do not have field names, just their types. However, it is better for code readability because the tuple now has a name, just like a struct.

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

In this case, `black` and `origin` are different types because they are instances of different structs, even though they have the same fields types. So a function taking an argument of type `Color` cannot take `Point` as an argument.

### Unit-Like Structs
Unit-like structs do not have any fields. These are called `unit-like` because they behave similar to `unit` tuples `()`. These can be useful when we want to define a trait on some type but do not have any data that we want to store in the type itself. `traits` are discussed in chapter 10. An example of unit-like structs is as follows

```
struct AlwaysEqual; // no need for parantheses.

fn main() {
    let subject = AlwaysEqual;
}
```

### Ownership of Struct data
In the `User` struct defined above, we used `String` rather than `&str` (string slice type). This is because we want each struct to own all of its data and for the data to be valid for as long as the entire struct is valid. It is possible for structs to store reference to data owned by something else but that requires the use of _lifetimes_, a Rust feature discussed in Chapter 10. If we modify the `User` struct to use `&str`, the compiler will throw an `expected named lifetime parameter` error. We'll learn how to fix these errors in Chapter 10.
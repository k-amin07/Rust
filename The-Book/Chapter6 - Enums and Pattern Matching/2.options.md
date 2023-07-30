Rust does not have a null feature that many other languages do. Null values are error prone, however, the concept of null is useful: a null is a value that is currently absent or invalid for some reason.

Rust does not have nulls but it does have an enum that can encode concepts of a value being present or absent. The enum is `Option<T>` in the standard library, defined as follows

```
enum Option<T> {
    None,
    Some(T)
}
```

This enum is so useful that we do not have to bring it into the scope explicitly, it is included in the prelude. Its variants are also included in the prelude, which means `None` and `Some` can be used without `Option::` prefix. 

The `<T>` syntax (remember from C++) is a generic type parameter. Generics are covered in Chapter 10.
```
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
```

The type of `some_number` is `Option<i32>`, the type of `some_char` is `Option<char>`. Rust can infer these types because we have specified a value inside the `Some` variant.

`absent_number` on the other hand requires us to annotate the overall `Option` type. The compiler cannot infer what value the corresponding `Some` variant will hold by looking only at a `None` value. Here, we tell Rust that we want `absent_number` to be of type `i32`.

However, we cannot add an `i32` to an `Option<i32>`. The following code wont work

```
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
```
The compiler will throw an error here. We have to convert `Option<T>` to the respective `T` before we can perform any operations with it. Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

`Options<T>` enum has a large number of methods that are useful in various situations. These methods can be found in the [documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
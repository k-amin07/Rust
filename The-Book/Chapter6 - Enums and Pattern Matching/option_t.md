In the previous section, we wanted to get the inner value of `Option<T>`. One way to do this is using `match` statement.

Lets say we want to have a function that accepts `Option<i32>` and if there is a value inside, returns an incremented value, otherwise returns `None`. [match_option_t](./projects/match_option_t/) demonstrates this case. We have to implement cases for `None` and `Some`. In our example, `None` returns `None`, `Some(i)` returns `Some(i+1)`.

This pattern is used extensively in Rust, it is a bit tricky at first, but once we get used to it, it is much safer than `null` in other languages.

One important thing to note here is that `match` is exhaustive. If we remove the None value from this example, compiler will throw an error.
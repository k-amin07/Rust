Extension of the guessing game. Here we use crates to install `rand` by adding it to dependencies in `Cargo.toml`. We can do this either by manually adding `rand = "0.8.5"` to `Cargo.toml` file or by simply using `cargo add rand` which will find the latest version of teh crate and add it to `Cargo.toml`.

`Cargo.lock` file ensures that the specified version of the crate is always fetched, making the code reproducible, even if a newer version of the package is available. This ensures that what is working today will continue to work in the future. Everytime we run cargo build, it will fetch the same version of the crates (if they are not locally available) unless a different version is explicitly specified.

`cargo update` command will look for updates and install the next available update under the same major version (for example 0.3.1 will be updated to 0.3.4 but not to 0.4.0).

In this module we add a random number generator using `rand`. We specify `use rand::Rng;` to use the random number generator of the `rand` module. The book specifies another syntax where we add `extern crate rand;` which is the equivalent of `use::rand`. `rand::thread_rng` gives us a particular random number generator which is thread specific, i.e. local to the current thread. Then we use the `gen_range` method of `thread_rng` to generate a random number from a range. Here my code deviates slightly from the book. Syntax in the book states
```
let secret_number = rand::thread_rng().gen_range(1,101);
```
However, this produces an error for me, this syntax may have changed in recent versions of rand module (book uses 0.3.x, we have 0.8.x). In the current version, the comma is replaced by `..` in gen_range arguments, i.e.
```
let secret_number = rand::thread_rng().gen_range(1..101);
```

Now, onto the game. We compare the number generated through the `rand` crate to the number entered by the user. For this purpose, we bring `Ordering` module into our scope throug `using std::cmp::Ordering;` statement. `Ordering` is an enum with `Less`, `Greater` and `Equal` variants. We use a `match` statement to decide what to do with the result. A match statement consists of _arms_. An _arm_ is a pattern and the code runs if the input expression matches the arm (kindof like an if-else or switch-case statements).

There is a problem in the code however, we declare input variable `guess` as string type, but we are comparing it with an integer (Rust defaults to i32 integer type here). This generates an error like this

```
error[E0308]: mismatched types
  --> src/main.rs:19:21
   |
19 |     match guess.cmp(&secret_number) {
   |                 --- ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`
   |                 |
   |                 arguments to this method are incorrect
   |
   = note: expected reference `&String`
              found reference `&{integer}`
```

The next concept here is variable _shadowing_. We add the line `let guess: u32 = guess.trim().parse().expect("Please enter a number!");`. Although we have already declared guess variable, shadowing lets us reuse the variable instead of creating two different variables. This is often used for type conversions.
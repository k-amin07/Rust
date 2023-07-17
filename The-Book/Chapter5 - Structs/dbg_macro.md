Another way to print a value using the `Debug` format is to use the `dbg!` macro. The `dbg!` macro prints to `stderr` rather than `stdout`. `dbg!` takes ownership of an expression, prints the file and line number of where the `dbg!` macro call occurs in the code alongwith the resultant value of the expression, and returns the ownership of the value. In the rectanges example, we can do 

```
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
```

and it will produce an output like this
```
 cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/rectangles`
[src/main.rs:10] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

Note that `#[derive(Debug)]` must be added to the `Rectangle` struct for this to work. Refer to [derived traits](./derived_traits.md).
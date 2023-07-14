### loop statement
The loop statement will run a block of code indefinitely (a.k.a infinite loop) unless explicitly stopped using a break statement
```
    loop {
        println!("Loop is running!")
    }
```
to break the loop, something like this can be used
```
    let a:i32 = 4
    loop {
        a = a - 1
        if a == 0 {
            break
        }
    }
```

### While loop
Alternatively, a while loop can be used
```
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
```

### for in loop
Rust also supports for loops
```
    let a = [10,20,30,40,50];
    for element in a.iter() {
        println!("the current element is {}", element);
    }
```
Rust also supports ranges in for loops
```
    for number in 1..4 {
        println!(number) // prints 1 2 3
    }
```
and reverse ranges
```
     for number in (1..4).rev() {
        println!(number) // prints 3,2,1
    }
```
In this module we add loops to the generating_random_numbers code to allow for multiple guesses. To do this, we enclose the code in a `loop` block and add a `break` statement as needed.

```
    loop {
        ...
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        ...
    }
```
The program will run in an infinite loop until the user guesses correctly. However, in this case, we need to handle invalid input in a way that the program does not crash. To do this, we update our string parse code to use `match` clause and add a `continue` statement in case of an error.

```
    ...
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    ...
```

At this point, since we are done with our testing, we delete the line that prints the secret number to make the game more reasonable.
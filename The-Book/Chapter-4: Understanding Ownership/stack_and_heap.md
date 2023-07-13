Like C/C++, Rust deals with stack and heap. In languages like Python and Javascript, we do not have to worry about dynamic memory as it is managed by the language itself at runtime. In Rust, however, whether a value resides on stack or on heap can have an impact on certain decisions. For example, just like C/C++, arrays are on stack by default, which is why their size cannot change. However vectors reside on heap and provide more flexibility.

Stack is LIFO, and hence very fast because new elements are placed on top of the stack and the top element is the only one that can be removed. Heap is less organized, when we put data on the heap, we ask the OS for the required space. The OS finds a big enough spot on the heap and assigns it to our program.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so we don’t run out of space are all problems that ownership addresses.

### Ownership Rules
- Each value in Rust has a variable thats called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value is dropped.


### Variable Scope
```
{                       // s is not valid here as it has not been declared.
    let s = "hello";    // s is valid from this point forward.
    ...                 // do stuff with s
}                       // the scope is now over, s is no longer valid.
```
### String type
The book illustrates the rules of ownership using the string type. String literals ("hello" in the example above) are immutable in Rust and therefore, we use the `String` type for many scenarios. This type is allocated on heap and hence can store an amount of text unknown to us at compile time.
```
    let s = String::from("hello");
```
This kid of string _can_ be mutated (keyword is can, since we can have this declared as non-mutable like in the above code).
```
    {
        let mut s = String::from("hello");
        s.push_str(" world!");
        println!("{}",s);
    }
    // s is no longer valid
```
The String `s` in this piece of code is mutable. The difference here is that this string is allocated on heap, the String::from function implementation requests the required memory from OS at runtime. In Rust, the memory is freed as soon as the variable goes out of scope. When the variable goes out of scope, Rust autmatically calls the `drop` function of the String type. This is equivalent to the destructor in C++, however, in C++ we'd often have to call the `delete` operator explicitly.

### Variable interaction
- **Move**:
    In rust
    ```
        let a = 2;
        let b = a;
    ```
    will copy the value of a into b. However, with String
    ```
        let a = String::from("hello");
        let b = a;
    ```
    `b` will point to the same instance of String. However, when both `a` and `b` go out of scope, they'll both try to free up the same memory, causing a double free error. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities. To ensure memory safety, Rust no longer considers `a` to be valid, and hence does not need to do anything when `a` goes out of scope. If we try to use `a`, we will get a `use of moved value` error, indicating `value used here after move` on the offending line. 
    This is similar to to "shallow copy" in C/C++ but it is called move in rust because the previous value is also invalidated. We read this as `a was moved to b`

- **Clone:**
    Clone method is used in Rust for deep copy.
    ```
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    ```
- **Stack Only Data - Copy**
    ```
        let x = 5;
        let y = x;
        println!("y: {}, x: {}",y,x);
    ```
    This code is valid because Rust copies data for values that reside on Stack. There is no difference between deep and shallow copy for this data since both variables are fixed sized which is known at compile time.
    Rust has a special annotation called the Copy trait that can be placed on types like integers that are stored on the stack. If a type has the Copy trait, an older variable is still usable after assignment. Rust won’t let us annotate a type with the Copy trait if the type, or any of its parts, has implemented the Drop trait. More on this in later chapters.
    Some types that are Copy (refer to documentation of others if unsure):
    - All the integer types, like u32.
    - The boolean type, bool, with values true and false.
    - All the floating point types, like f64.
    - Tuples, but only if they contain types that are also Copy. (i32,i32) is Copy, but (i32, String) is not.


Although the variables in rust are immutable by default, they are still different from constants. In rust, variables are declared using `let` statement and constants are declared using `const` statement, like in javascript. Constants have a few properties.
- we cannot use mut with constants
- we _must_ annotate the type of the constant.
- constants can only be set to a constant value, not the result of a function call. These are compile time constants, not runtime constants.
- The constant naming convention is to use all uppercase letters with underscores.
    ```
        const MAX_POINTS: u32 = 100_000;
    ```
- Variables are not mutable by default, but if we need to update a non mutable variable later, we must redeclare (shadow) it. For example:
    ```
        let x = 5;
        let x = x + 1;
        let x = x * 2;
    ```
- We can use shadowing to use the same name but change the type of the variable.
    ```
        let spaces = "     ";
        let spaces = spaces.len();
    ```
- However, if we try to change the type of a mutable variable, we get a compile time error
    ```
        let mut spaces = "     ";
        spaces = spaces.len();
    ```
- Rust is a statically typed language, which means we must know the type of the variable at compile time.
    ```
        let guess: u32 = "42".parse().expect("Not a number!");  // works fine
        let guess = "42".parse().expect("Not a number!");       // raises a compile time error.
    ```
- Rust has four primary scalar types; float, int, bool and char. Int can be signed or unsigned and can be 8-bit (i8 / u8), 16-bit (i16 / u16), 32-bit (i32 / u32), 64-bit (i64 / u64) or arch (isize / usize).
    - arch means based on the CPU architecture (usually 32 or 64 bit). 
    - Each type can store numbers from (-2<sup>n-1</sup>) to (2<sup>n-1</sup> - 1).
    - Unsigned types can store numbers from 0 to 2<sup>n</sup> - 1
    - An i8 can store numbers from -128 to 127 and a u8 can store numbers from 0 to 255.

- Rust uses 64 bit floats by default, however, 32 bit floats can also be used. f32 is a single precision float while f64 is a double precision float.
    ```
    fn main() {
        let x = 2.0; // f64
        let y: f32 = 3.0; // f32
    }
    ```
- Rust supports arithemetic (+,-,*,/) operators as well as the mod (%) operator.
- Rust also has bool type to represent Booleans, they can either be true or false.
- The char type in Rust represents a Unicode scalar value, i.e. it can represent a lot more than ASCII characters.

Rust also supports compound types like tuples and arrays. Tuples can be declared as follows
```
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
The variable tup binds to the entire tuple since tuple is considered a single compound element. It is not necessary to add type annotations but probably should be done for consistency. Individual elements can be extracted by destructuring a tuple as follows
```
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}
```
The elements of a tuple can be referenced directly using `.` operator
```
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
```
On the other hand, all elements of an array must be of the same type.
```
    let a = [1,2,3,4,5]; // defaults to i32
```
Arrays are created on the stack, rather than the heap (static memory vs dynamic memory). Which means they are not very flexible. Vectors (like in C++) are dynamic and are allowed to shrink or expand.
Array elements are accessed through indexing
```
    let first = a[0];
    let second = a[1];
```
Trying to access the 10th element of this array, for example, will produce an index out of bounds runtime error.
Rust uses snake_case as the conventional style for function and variable names.
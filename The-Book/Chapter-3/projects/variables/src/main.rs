fn main() {
    // let x = 5; // this wont let us change value later, unless we shadow it
    let mut x = 5; // this will let us change value later
    println!("The value of x is {}.",x);
    x = 6;
    println!("The value of x is {}.",x);
}

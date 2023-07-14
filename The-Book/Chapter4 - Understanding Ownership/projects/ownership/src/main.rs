fn main() {
    let s = String::from("hello");
    takes_ownersip(s);

    // println!("{}",s); // wont work because s is out of scope
    // ^ we get borrow of moved value: `s` error

    let x = 5;
    makes_copy(x);
    println!("x in main: {}",x);
    // works fine because integers are passed by copy.
}

fn takes_ownersip(some_string:String) {
    println!("{}",some_string);
}

fn makes_copy(some_integer:i32) {
    println!("some_integer in makes_copy: {}",some_integer);
}
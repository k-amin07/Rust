fn main() {
    let str1 = String::from("hello");

    {
        let mut s = String::from("hello");
        s.push_str(" world!");
        println!("{}",s);
    } // s goes out of scope here

    // println!("{}",s); // will produce not found in this scope error
    
    let str2 = str1; // str2 and str1 point to same object
    // str1 is no longer considered valid
    // println!("{}",str1) // will produce an error

    let str3 = str2.clone(); // deep copy
    // str3 is now valid, so is str2;
    println!("{}\n{}",str2,str3);

    let x = 5;
    let y = x;
    println!("y: {}, x: {}",y,x);
    // this is valid because both x and y reside on stack so it is always copied
}

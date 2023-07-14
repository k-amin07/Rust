fn main() {
    let s1 = gives_ownership(); // return value of function moved into s1
    println!("s1: {}",s1);
    let s2 = String::from("abc"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); //s2 moves out of scope and s3 moves into scope
    // println!("{}",s2); // will throw value borrowed after move error
    println!("s3: {}",s3);
    let (s4,len) = calculate_length(s3);
    println!("s4: {}, length: {}",s4,len)
}

fn gives_ownership()-> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// this function returns the string and its length
fn calculate_length(s:String) -> (String,usize) {
    let length = s.len();
    (s,length)
}
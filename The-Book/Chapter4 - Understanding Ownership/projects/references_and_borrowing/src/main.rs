fn main() {
    let s1 = String::from("hello");

    let (s2,len) = calculate_length(s1);
    println!("s2: {}, length: {}",s2,len);

    let len2 = calculate_length_by_ref(&s2);
    println!("s2: {}, length: {}",s2,len2);

    let mut s3 = s2.clone();
    change(&mut s3); // s3 must me mutable, and its reference must also be mutable
    println!("s3: {}",s3);
}

fn calculate_length(s:String) -> (String,usize) {
    let length = s.len();
    (s,length)
}

fn calculate_length_by_ref(s:&String) -> usize {
    s.len()
}

fn change(s: &mut String) { //must accept a mutable reference
    s.push_str(" abc")
}
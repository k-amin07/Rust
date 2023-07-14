fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("{}",word);
    s.clear();

    
    let mut s = String::from("hello world");
    let word2 = first_word_slice(&s);
    s.clear();  
    // println!("{}",word2); // uncomment this line for "cannot borrow `s` as mutable because it is also borrowed as immutable" error

    // ----------------------------------- //

    // The following code demonstrates the flexibility achieved through using string literals in function def
    // directly copied from the book (section 4.3)
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_literal(&my_string[0..6]);
    let word = first_word_literal(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_literal(&my_string);

    let my_string_literal = "hello world";
    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_literal(&my_string_literal[0..6]);
    let word = first_word_literal(&my_string_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_literal(my_string_literal);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s:&String)->&str {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word_literal(s:&str)->&str {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
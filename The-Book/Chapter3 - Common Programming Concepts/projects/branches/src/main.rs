fn main() {
    let number = 7;
    if number < 3 {
        println!("Condtion was True");
    } else {
        println!("Condition was False");
    }
    let second_number = if number < 3 {
        7
    } else {
        3
    };
    println!("Second Number is {}", second_number);
}

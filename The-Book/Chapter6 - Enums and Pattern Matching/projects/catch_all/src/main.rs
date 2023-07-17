fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => func1(),
        4 => func2(),
        other => func3(),
    }
    match dice_roll {
        3 => func1(),
        4 => func2(),
        _ => func3(),
    }
    match dice_roll {
        3 => func1(),
        4 => func2(),
        _ => (),
    }
}

fn func1() {
    println!("three");
}

fn func2() {
    println!("four");
}

fn func3() {
    println!("other");
}
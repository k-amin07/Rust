#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // -- other 48 states go below
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main () {
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    match coin { 
        // coin move here, cannot be used later in code
        // can however use &coin
        Coin::Quarter(state)=>println!("State Quarter from {:#?}!",state),
        _ => count += 1,
    }
    dbg!(count);

    let coin2 = Coin::Penny;
    if let Coin::Quarter(state) = coin2 {
        println!("State Quarter from {:#?}!",state);
    } else {
        count += 1;
    }

    dbg!(count);
}
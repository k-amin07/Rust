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

fn value_in_cents(coin:&Coin) -> i32 {
    match coin {
        Coin::Penny =>  {
            println!("Oh look, a penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:#?}", state);
            25
        }
    }

}

fn main() {
    let coin = Coin::Penny;
    println!("Value: {}",value_in_cents(&coin));

    let coin2 = Coin::Quarter(UsState::Alabama);
    println!("Value: {}",value_in_cents(&coin2));
}

enum Coin {
    Penny,
    Nickey,
    Dime,
    Quarter,
}

fn value_in_cents (coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickey => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("Hello, world!");
}

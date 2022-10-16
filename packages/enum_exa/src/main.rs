enum Coin {
    Panny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn main() {
    let number_mem = Some(3);
    let num = 3;
    // cannot add `Option<{integer}>` to `{integer}`
    // println!("{}", num + number_mem);

    let coin1 = value_with_coin(Coin::Quarter(UsState::Alabama));
    dbg!(coin1);

    let config_max = Some(255u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
    } else {
    }
}
fn value_with_coin(coin: Coin) -> i32 {
    match coin {
        Coin::Panny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            dbg!(state);
            25
        }
    }
}

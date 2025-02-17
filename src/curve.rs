const INITIAL_VIRTUAL_X: u128 = 1000;
const INITIAL_VIRTUAL_Y: u128 = 1000;

pub fn calculate_buy_price(amount: u128) -> u128 {
    let k = INITIAL_VIRTUAL_X * INITIAL_VIRTUAL_Y;
    let new_x = INITIAL_VIRTUAL_X + amount;
    let new_y = k / new_x;
    (k / new_y) - (k / (new_y + amount))
}

pub fn calculate_sell_price(amount: u128) -> u128 {
    let k = INITIAL_VIRTUAL_X * INITIAL_VIRTUAL_Y;
    let new_x = INITIAL_VIRTUAL_X - amount;
    let new_y = k / new_x;
    (k / (new_y - amount)) - (k / new_y)
}
use std::f32::INFINITY;

fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount <= 0 {
        return 0;
    }
    let mut coin_map = vec![];
    for _ in 0..amount + 1 {
        coin_map.push(INFINITY)
    }
    coin_map[0] = 0.0;
    let mut max_value;
    for coin in coins {
        for i in coin..amount + 1 {
            if i > coin {
                max_value = coin_map[(i - coin) as usize] + 1.0;
            } else {
                max_value = coin_map[(coin - i) as usize] + 1.0;
            }
            coin_map[i as usize] = f32::min(coin_map[i as usize], max_value);
        }
    }
    if coin_map[amount as usize] == f32::INFINITY {
        return -1;
    }
    coin_map[amount as usize] as i32
}
fn main() {
    let coins = vec![1, 2, 5];
    let amount = 11;
    let res = coin_change(coins, amount);
    println!("{}", res)
}

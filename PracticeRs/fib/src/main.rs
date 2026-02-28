use std::collections::HashMap;

fn fib_recr(n: u32, map: &mut HashMap<u32, u64>) -> u64 {
    if n < 2 {
        return n as u64;
    }
    if map.contains_key(&n) {
        return map[&n];
    }

    let res = fib_recr(n - 1, map) + fib_recr(n - 2, map);
    map.insert(n, res);
    res
}
fn fib(n: u32) -> u64 {
    let mut map = HashMap::<u32, u64>::new();
    fib_recr(n, &mut map)
}
fn main() {
    let res = fib(71);
    println!("Fib result : {}", res);
}

use std::collections::HashMap;

fn comb_recr(n: u32, r: u32, map: &mut HashMap<(u32, u32), u64>) -> u64 {
    if r == 0 || r == n {
        return 1;
    }
    if map.contains_key(&(n, r)) {
        return map[&(n, r)];
    }

    let res = comb_recr(n - 1, r - 1, map) + comb_recr(n - 1, r, map);
    map.insert((n, r), res);

    res
}

fn combination(n: u32, r: u32) -> u64 {
    let mut map = HashMap::<(u32, u32), u64>::new();
    comb_recr(n, r, &mut map)
}
fn main() {
    let res = combination(100, 10);

    println!("Result C(100, 10) : {}", res);
}

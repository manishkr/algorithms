pub fn reverse(x: i32) -> i32 {
    let mut input = x;
    let mut rev = 0;
    while input != 0 {
        let reminder = input % 10;
        input /= 10;

        if rev > i32::MAX / 10 || (rev == i32::MAX && reminder > 7) {
            return 0;
        }
        if rev < i32::MIN / 10 || (rev == i32::MIN && reminder < -8) {
            return 0;
        }
        rev = rev * 10 + reminder;
    }

    return rev;
}

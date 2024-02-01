pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    for (pos_1, e_1) in nums.iter().enumerate() {
        for (pos_2, e_2) in nums[pos_1 + 1..].iter().enumerate() {
            if e_1 + e_2 == target {
                v.push(pos_1 as i32);
                v.push((pos_1 + 1 + pos_2) as i32);
                break;
            }
        }
    }
    return v;
}

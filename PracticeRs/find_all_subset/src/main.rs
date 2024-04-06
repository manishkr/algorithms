fn find_all_subset(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    result.push(Vec::new());
    for item in nums {
        let mut new_subsets = Vec::new();
        for subset in result.iter() {
            let mut subset_clone = subset.clone();
            subset_clone.push(item);
            new_subsets.push(subset_clone);
        }
        result.extend(new_subsets);
    }
    result
}
fn main() {
    let set = vec![1, 2, 3];
    println!("{:?}", find_all_subset(set))
}

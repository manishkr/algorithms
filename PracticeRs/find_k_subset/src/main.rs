use std::collections::HashSet;

fn subsets_of_size_k(set: &HashSet<i32>, k: usize) -> Vec<HashSet<i32>> {
    let mut result = Vec::new();
    let mut current_set = HashSet::new();
    let elements: Vec<i32> = set.iter().cloned().collect();
    generate_subsets(&elements, 0, k, &mut current_set, &mut result);
    result
}
fn generate_subsets(
    elements: &[i32],
    start: usize,
    k: usize,
    current_set: &mut HashSet<i32>,
    result: &mut Vec<HashSet<i32>>,
) {
    if current_set.len() == k {
        result.push(current_set.clone());
    }
    for i in start..elements.len() {
        current_set.insert(elements[i]);
        generate_subsets(elements, start + 1, k, current_set, result);
        current_set.remove(&elements[i]);
    }
}
fn main() {
    let set = HashSet::from([11, 1, 2, 3]);
    let subsets = subsets_of_size_k(&set, 2);
    println!("{:?}", subsets);
}

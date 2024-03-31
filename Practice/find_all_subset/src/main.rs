use std::collections::HashSet;

fn find_subsets(set: &HashSet<i32>) -> Vec<HashSet<i32>> {
    let mut subsets = Vec::new();
    subsets.push(HashSet::new());

    for &item in set.iter() {
        let mut new_subsets = Vec::new();
        for subset in subsets.iter() {
            let mut clone_subset = subset.clone();
            clone_subset.insert(item);
            new_subsets.push(clone_subset);
        }
        subsets.extend(new_subsets);
    }
    subsets
}

fn main() {
    let set = HashSet::from([1, 2, 3, 4, 5]);

    let subsets = find_subsets(&set);
    println!("length of subsets {}", subsets.len());
    for subset in subsets {
        println!("{:?}", subset);
    }
}

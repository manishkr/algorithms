use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn find_k_min(list: &[i32], k: usize) -> Vec<i32> {
    if k == 0 {
        return vec![];
    }
    let mut heap = BinaryHeap::new();

    for item in list {
        heap.push(item);
        if heap.len() == k {
            break;
        }
    }
    if k <= list.len() {
        for item in &list[k..] {
            if let Some(top) = heap.peek() {
                if *top > item {
                    heap.pop();
                    heap.push(item);
                }
            }
        }
    }
    let res = heap.into_iter().map(|x| *x).collect();
    res
}

fn find_k_max(list: &[i32], k: usize) -> Vec<i32> {
    if k == 0 {
        return vec![];
    }
    let mut heap = BinaryHeap::new();

    for item in list {
        heap.push(Reverse(item));
        if heap.len() == k {
            break;
        }
    }
    if k <= list.len() {
        for item in &list[k..] {
            if let Some(Reverse(top)) = heap.peek() {
                if *top < item {
                    heap.pop();
                    heap.push(Reverse(item));
                }
            }
        }
    }
    let mut res = vec![];
    while let Some(Reverse(num)) = heap.pop() {
        res.push(*num)
    }
    res
}

fn main() {
    let list = [2, 4, 1, 4, 2, 3, 12, 11, 22, 222, -1];
    let k = 3;
    let res = find_k_min(&list, k);
    println!("{:?}", res);

    let k = 3;
    let res = find_k_max(&list, k);
    println!("{:?}", res);

    let k = 3;
    let res = find_k_max(&[], k);
    println!("{:?}", res);

    let k = 3;
    let res = find_k_max(&[1, 2, 3], k);
    println!("{:?}", res);

    let k = 1;
    let res = find_k_max(&[1, 2, 3], k);
    println!("{:?}", res);

    let k = 0;
    let res = find_k_max(&[1, 2, 3], k);
    println!("{:?}", res)
}

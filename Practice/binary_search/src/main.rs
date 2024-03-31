fn binary_search(list: &[i32], num: i32) -> Option<usize> {
    let mut l = 0usize;
    let mut r = list.len() - 1;
    while l <= r {
        if l == 0 && r == 0 && list[0] != num {
            return None;
        }
        let mid = r - (r - l) / 2;
        if list[mid] == num {
            return Some(mid);
        } else if num < list[mid] {
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    None
}

fn main() {
    let list = [1, 2, 3, 7, 9, 11, 15, 23, 28];
    let res = binary_search(&list, 9);
    println!("{:?}", res);

    let res = binary_search(&list, 12);
    println!("{:?}", res);

    let res = binary_search(&list, 1111);
    println!("{:?}", res);

    let res = binary_search(&list, -1111);
    println!("{:?}", res);

    let res = binary_search(&list, 28);
    println!("{:?}", res);

    let res = binary_search(&list, 1);
    println!("{:?}", res);

    let res = binary_search(&list, 11);
    println!("{:?}", res);
}

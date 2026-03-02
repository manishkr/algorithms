use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Stock {
    id: u32,
    stock: i32,
}

impl Ord for Stock {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.stock
            .cmp(&other.stock)
            .then(self.id.cmp(&other.id))
    }
}

impl PartialOrd for Stock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn running_max(product_ids: &[u32], inventory_changes: &[i32]) -> Vec<u32> {
    let mut stock_map = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut result = Vec::new();

    for (&pid, &delta) in product_ids.iter().zip(inventory_changes) {
        let entry = stock_map.entry(pid).or_insert(0);
        *entry += delta;

        heap.push(Stock { id: pid, stock: *entry });

        while let Some(top) = heap.peek() {
            if stock_map[&top.id] != top.stock {
                heap.pop();
            } else {
                break;
            }
        }

        if let Some(top) = heap.peek() {
            result.push(top.stock as u32);
        }
    }

    result
}

fn main() {
    let product_ids = vec![101, 102, 101, 105];
    let inventory_changes = vec![4, 2, -4, 3];
    println!("{:?}", running_max(&product_ids, &inventory_changes));
}

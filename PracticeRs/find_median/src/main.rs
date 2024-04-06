use std::cmp::Reverse;
use std::collections::BinaryHeap;
struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.max_heap.is_empty() || num <= *self.max_heap.peek().unwrap() {
            self.max_heap.push(num)
        } else {
            self.min_heap.push(Reverse(num));
        }

        // Balance the heaps
        if self.max_heap.len() > self.min_heap.len() + 1 {
            if let Some(max) = self.max_heap.pop() {
                self.min_heap.push(Reverse(max));
            }
        } else if self.min_heap.len() > self.max_heap.len() {
            if let Some(Reverse(min)) = self.min_heap.pop() {
                self.max_heap.push(min);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.max_heap.len() == self.min_heap.len() {
            (self.max_heap.peek().unwrap() + self.min_heap.peek().unwrap().0) as f64 / 2.0
        } else {
            *self.max_heap.peek().unwrap() as f64
        }
    }
}

fn main() {
    let mut median_finder = MedianFinder::new();
    let nums = vec![2, 1, 4, 5, 3];
    for num in nums {
        median_finder.add_num(num);
    }
    println!("Median: {}", median_finder.find_median()); // Output: 3.0
}

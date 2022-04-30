use std::cmp;

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
  let mut shortest_vec = &nums1;
  let mut longest_vec = &nums2;

  if nums1.len() > nums2.len() {
    longest_vec = &nums1;
    shortest_vec = &nums2;
  }

  let (mut min, mut max) = (0usize, shortest_vec.len());
  let (mut i, mut j) = (0usize, 0usize);
  let mut median = 0;
  let mid = (shortest_vec.len() + longest_vec.len() + 1) / 2;
  while min <= max {
    i = (min + max) / 2;
    j = mid - i;
    if i < shortest_vec.len() && j > 0 && longest_vec[j - 1] > shortest_vec[i] {
      min = i + 1;
    } else if i > 0 && j < longest_vec.len() && longest_vec[j] < shortest_vec[i - 1] {
      max = i - 1
    } else {
      if i == 0 {
        median = longest_vec[j - 1];
      } else if j == 0 {
        median = shortest_vec[i - 1];
      } else {
        median = cmp::max(shortest_vec[i - 1], longest_vec[j - 1]);
      }
      break;
    }
  }

  if (shortest_vec.len() + longest_vec.len()) % 2 == 1 {
    median as f64
  } else if i == shortest_vec.len() {
    (median + longest_vec[j]) as f64 / 2.0
  } else if j == longest_vec.len() {
    (median + shortest_vec[i]) as f64 / 2.0
  } else {
    (median + cmp::min(shortest_vec[i], longest_vec[j])) as f64 / 2.0
  }
}

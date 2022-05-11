use std::cmp::{max, min};

pub fn max_area(height: Vec<i32>) -> i32 {
  let mut trap_area = 0;
  let mut left_index = 0usize;
  let mut right_index = height.len() - 1;

  while left_index < right_index {
    let width = (right_index - left_index) as i32;
    trap_area = max(trap_area, min(height[left_index], height[right_index]) * width);
    if height[left_index] < height[right_index] {
      left_index += 1;
    } else {
      right_index -= 1;
    }
  }
  return trap_area;
}

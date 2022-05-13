pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
  let mut result: Vec<Vec<i32>> = Vec::new();
  let mut sorted_input = nums.clone();
  sorted_input.sort();

  for (index, element) in sorted_input.iter().enumerate() {
    if index > 0 && *element == sorted_input[index - 1] {
      continue;
    }
    let mut left_index = index + 1;
    let mut right_index = sorted_input.len() - 1;
    while left_index < right_index {
      let sum_value = *element + sorted_input[left_index] + sorted_input[right_index];
      if sum_value > 0 {
        right_index -= 1;
      } else if sum_value < 0 {
        left_index += 1;
      } else {
        result.append(&mut vec![vec![*element, sorted_input[left_index], sorted_input[right_index]]]);
        left_index += 1;
        while sorted_input[left_index] == sorted_input[left_index - 1] && left_index < right_index {
          left_index += 1;
        }
      }
    }
  }
  return result;
}

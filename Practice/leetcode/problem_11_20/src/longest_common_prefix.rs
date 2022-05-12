fn is_common_prefix(strs: &Vec<String>, length: usize) -> bool {
  let string = &strs[0][0..length];
  for item in strs.iter().skip(1) {
    if !item.starts_with(string) {
      return false;
    }
  }
  return true;
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
  if strs.len() == 0 {
    return "".to_string();
  }
  if strs.len() == 1 {
    return strs[0].to_string();
  }

  let min_length = strs.iter().min_by(|a, b|
    a.len().cmp(&b.len())).unwrap().len();

  let mut low_index = 1;
  let mut high_index = min_length;

  while low_index <= high_index {
    let middle = (high_index - low_index) / 2 + low_index;
    if is_common_prefix(&strs, middle) {
      low_index = middle + 1;
    } else {
      high_index = middle - 1;
    }
  }
  let index = (high_index + low_index) / 2;
  return strs[0][0..index].to_string();
}

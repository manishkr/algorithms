fn expand_around_center(s: &Vec<char>, left: usize, right: usize) -> usize {
  let (mut l, mut r) = (left as i32, right as i32);

  while l >= 0 && (r as usize) < s.len() && s[l as usize] == s[r as usize] {
    l = l - 1;
    r = r + 1;
  }

  return (r - l - 1) as usize;
}

pub fn longest_palindrome(s: String) -> String {
  if s.len() < 1 {
    return "".to_owned();
  }

  let (mut start, mut end) = (0, 0);
  let mut max_length = 0;

  let chars = s.chars().collect();

  for i in 0..s.len() {
    let mut length = expand_around_center(&chars, i, i);
    if length > max_length {
      max_length = length;
      start = i - length / 2;
      end = i + length / 2;
    }

    length = expand_around_center(&chars, i, i + 1);
    if length > max_length {
      max_length = length;
      start = i - (length - 1) / 2;
      end = i + length / 2;
    }
  }
  return s[start..end + 1].to_owned();
}

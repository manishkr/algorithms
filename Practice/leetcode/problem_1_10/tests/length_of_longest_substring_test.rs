#[cfg(test)]
mod tests {
  use problem_1_10::length_of_longest_substring;

  #[test]
  fn test_length_of_longest_substring_1() {
    assert_eq!(
      length_of_longest_substring::length_of_longest_substring("".to_owned()),
      0
    );
  }

  #[test]
  fn test_length_of_longest_substring_2() {
    assert_eq!(
      length_of_longest_substring::length_of_longest_substring("abcabcbb".to_owned()),
      3
    );
  }

  #[test]
  fn test_length_of_longest_substring_3() {
    assert_eq!(
      length_of_longest_substring::length_of_longest_substring("bbbbb".to_owned()),
      1
    );
  }

  #[test]
  fn test_length_of_longest_substring_4() {
    assert_eq!(
      length_of_longest_substring::length_of_longest_substring("pwwkew".to_owned()),
      3
    );
  }

  #[test]
  fn test_length_of_longest_substring_5() {
    assert_eq!(
      length_of_longest_substring::length_of_longest_substring("pqrstuv".to_owned()),
      7
    );
  }
}

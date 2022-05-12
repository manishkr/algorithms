#[cfg(test)]
mod tests {
  use problem_11_20::longest_common_prefix;

  #[test]
  fn test_longest_common_prefix_1() {
    assert_eq!(longest_common_prefix::longest_common_prefix(vec!["dog".to_string(),
                                                                 "racecar".to_string(), "car".to_string()]), "");
  }

  #[test]
  fn test_longest_common_prefix_2() {
    assert_eq!(longest_common_prefix::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl");
  }

  #[test]
  fn test_longest_common_prefix_3() {
    assert_eq!(longest_common_prefix::longest_common_prefix(vec!["Test".to_string(), "Test".to_string(), "Test".to_string()]), "Test");
  }

  #[test]
  fn test_longest_common_prefix_4() {
    assert_eq!(longest_common_prefix::longest_common_prefix(vec!["Test".to_string(), "test".to_string()]), "");
  }

  #[test]
  fn test_longest_common_prefix_5() {
    assert_eq!(longest_common_prefix::longest_common_prefix(vec!["Test".to_string()]), "Test");
  }
}

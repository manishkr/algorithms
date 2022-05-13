#[cfg(test)]
mod tests {
  use problem_11_20::longest_common_prefix;

  #[test]
  fn test_longest_common_prefix_1() {
    assert_eq!(longest_common_prefix::longest_common_prefix(vec!["dog".to_owned(),
                                                                 "racecar".to_owned(), "car".to_owned()]), "");
  }

  #[test]
  fn test_longest_common_prefix_2() {
    assert_eq!(longest_common_prefix::longest_common_prefix(vec!["flower".to_owned(), "flow".to_owned(), "flight".to_owned()]), "fl");
  }

  #[test]
  fn test_longest_common_prefix_3() {
    assert_eq!(longest_common_prefix::longest_common_prefix(vec!["Test".to_owned(), "Test".to_owned(), "Test".to_owned()]), "Test");
  }

  #[test]
  fn test_longest_common_prefix_4() {
    assert_eq!(longest_common_prefix::longest_common_prefix(vec!["Test".to_owned(), "test".to_owned()]), "");
  }

  #[test]
  fn test_longest_common_prefix_5() {
    assert_eq!(longest_common_prefix::longest_common_prefix(vec!["Test".to_owned()]), "Test");
  }
}

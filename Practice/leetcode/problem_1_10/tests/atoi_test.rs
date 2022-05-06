#[cfg(test)]
mod tests {
  use problem_1_10::atoi;

  #[test]
  fn test_atoi_1() {
    assert_eq!(atoi::my_atoi("23".to_owned()), 23);
  }

  #[test]
  fn test_atoi_2() {
    assert_eq!(atoi::my_atoi("-23".to_owned()), -23);
  }

  #[test]
  fn test_atoi_3() {
    assert_eq!(atoi::my_atoi("2367 with words".to_owned()), 2367);
  }

  #[test]
  fn test_atoi_4() {
    assert_eq!(atoi::my_atoi("-91283472332".to_owned()), i32::MIN);
  }

  #[test]
  fn test_atoi_5() {
    assert_eq!(atoi::my_atoi("2147483648".to_owned()), i32::MAX);
  }
}

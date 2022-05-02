#[cfg(test)]
mod tests {
  use problem_1_10::zigzag_conversion;

  #[test]
  fn test_length_of_longest_substring_1() {
    assert_eq!(zigzag_conversion::convert("A".to_owned(), 1), "A");
  }

  #[test]
  fn test_length_of_longest_substring_2() {
    assert_eq!(zigzag_conversion::convert("PAYPALISHIRING".to_owned(), 3), "PAHNAPLSIIGYIR");
  }

  #[test]
  fn test_length_of_longest_substring_3() {
    assert_eq!(zigzag_conversion::convert("PAYPALISHIRING".to_owned(), 4), "PINALSIGYAHRPI");
  }
}

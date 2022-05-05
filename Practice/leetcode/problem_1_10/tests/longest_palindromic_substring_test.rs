#[cfg(test)]
mod tests {
  use problem_1_10::longest_palindromic_substring;

  #[test]
  fn test_longest_palindromic_substring_1() {
    assert_eq!(
      longest_palindromic_substring::longest_palindrome("".to_owned()),
      ""
    );
  }

  #[test]
  fn test_longest_palindromic_substring_2() {
    assert_eq!(
      longest_palindromic_substring::longest_palindrome("babad".to_owned()),
      "bab"
    );
  }

  #[test]
  fn test_longest_palindromic_substring_3() {
    assert_eq!(
      longest_palindromic_substring::longest_palindrome("cbbd".to_owned()),
      "bb"
    );
  }

  #[test]
  fn test_longest_palindromic_substring_4() {
    assert_eq!(
      longest_palindromic_substring::longest_palindrome("bbbb".to_owned()),
      "bbbb"
    );
  }

  #[test]
  fn test_longest_palindromic_substring_5() {
    assert_eq!(
      longest_palindromic_substring::longest_palindrome("pqrstuv".to_owned()),
      "p"
    );
  }

  #[test]
  fn test_longest_palindromic_substring_6() {
    assert_eq!(
      longest_palindromic_substring::longest_palindrome("ccc".to_owned()),
      "ccc"
    );
  }

  #[test]
  fn test_longest_palindromic_substring_7() {
    assert_eq!(longest_palindromic_substring::longest_palindrome("dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd".to_owned()), "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd");
  }
}

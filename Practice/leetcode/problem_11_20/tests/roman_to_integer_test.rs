#[cfg(test)]
mod tests {
  use problem_11_20::roman_to_integer;

  #[test]
  fn test_roman_to_integer_1() {
    assert_eq!(roman_to_integer::roman_to_int("III".to_owned()), 3);
  }

  #[test]
  fn test_roman_to_integer_2() {
    assert_eq!(roman_to_integer::roman_to_int("LVIII".to_owned()), 58);
  }

  #[test]
  fn test_roman_to_integer_3() {
    assert_eq!(roman_to_integer::roman_to_int("MCMXCIV".to_owned()), 1994);
  }

  #[test]
  fn test_roman_to_integer_4() {
    assert_eq!(roman_to_integer::roman_to_int("IV".to_owned()), 4);
  }

  #[test]
  fn test_roman_to_integer_5() {
    assert_eq!(roman_to_integer::roman_to_int("XL".to_owned()), 40);
  }
}

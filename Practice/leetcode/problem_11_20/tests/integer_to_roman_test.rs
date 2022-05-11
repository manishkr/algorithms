#[cfg(test)]
mod tests {
  use problem_11_20::integer_to_roman;

  #[test]
  fn test_integer_to_roman_1() {
    assert_eq!(integer_to_roman::int_to_roman(3), "III");
  }

  #[test]
  fn test_integer_to_roman_2() {
    assert_eq!(integer_to_roman::int_to_roman(58), "LVIII");
  }

  #[test]
  fn test_integer_to_roman_3() {
    assert_eq!(integer_to_roman::int_to_roman(1994), "MCMXCIV");
  }

  #[test]
  fn test_integer_to_roman_4() {
    assert_eq!(integer_to_roman::int_to_roman(4), "IV");
  }

  #[test]
  fn test_integer_to_roman_5() {
    assert_eq!(integer_to_roman::int_to_roman(40), "XL");
  }
}

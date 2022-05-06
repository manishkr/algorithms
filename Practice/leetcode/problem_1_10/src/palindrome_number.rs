pub fn is_palindrome(x: i32) -> bool {
  if x < 0 || x % 10 == 0 && x != 0 {
    return false;
  }

  let mut input = x;
  let mut reverted_number = 0;
  while input > reverted_number {
    reverted_number = reverted_number * 10 + input % 10;
    input /= 10;
  }
  return input == reverted_number || input == reverted_number / 10;
}

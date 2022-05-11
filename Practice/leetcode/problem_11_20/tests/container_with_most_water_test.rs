#[cfg(test)]
mod tests {
  use problem_11_20::container_with_most_water;

  #[test]
  fn test_container_with_most_water_1() {
    assert_eq!(container_with_most_water::max_area(vec![1, 0]), 0);
  }

  #[test]
  fn test_container_with_most_water_2() {
    assert_eq!(container_with_most_water::max_area(vec![1, 1]), 1);
  }

  #[test]
  fn test_container_with_most_water_3() {
    assert_eq!(container_with_most_water::max_area(vec![5, 2]), 2);
  }

  #[test]
  fn test_container_with_most_water_4() {
    assert_eq!(container_with_most_water::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
  }

  #[test]
  fn test_container_with_most_water_5() {
    assert_eq!(container_with_most_water::max_area(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
               14);
  }
}

#[cfg(test)]
mod tests {
  use problem_11_20::three_sum;

  #[test]
  fn test_three_sum_1() {
    assert_eq!(three_sum::three_sum(vec![-1, 0, 1, 2, -1, 4]), vec![vec![-1, -1, 2],
                                                                    vec![-1, 0, 1]]);
  }

  #[test]
  fn test_three_sum_2() {
    assert_eq!(three_sum::three_sum(vec![1, -1, 0]), vec![vec![-1, 0, 1]]);
  }

  #[test]
  fn test_three_sum_3() {
    assert_eq!(three_sum::three_sum(vec![5, 2, 7]), Vec::<Vec<i32>>::new());
  }

  #[test]
  fn test_three_sum_4() {
    assert_eq!(three_sum::three_sum(Vec::<i32>::new()), Vec::<Vec<i32>>::new());
  }

  #[test]
  fn test_three_sum_5() {
    assert_eq!(three_sum::three_sum(vec![0]), Vec::<Vec<i32>>::new());
  }
}

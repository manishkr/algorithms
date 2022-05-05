#[cfg(test)]
mod tests {
  use problem_1_10::median_of_sorted_arrays;

  #[test]
  fn median_of_sorted_arrays_1() {
    assert_eq!(
      median_of_sorted_arrays::find_median_sorted_arrays(vec![1, 3], vec![2]),
      2.0
    );
  }

  #[test]
  fn median_of_sorted_arrays_2() {
    assert_eq!(
      median_of_sorted_arrays::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
      2.5
    );
  }

  #[test]
  fn median_of_sorted_arrays_3() {
    assert_eq!(
      median_of_sorted_arrays::find_median_sorted_arrays(
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        vec![1, 2, 3, 4],
      ),
      3.5
    );
  }

  #[test]
  fn median_of_sorted_arrays_4() {
    assert_eq!(
      median_of_sorted_arrays::find_median_sorted_arrays(vec![1], vec![]),
      1.0
    );
  }

  #[test]
  fn median_of_sorted_arrays_5() {
    assert_eq!(
      median_of_sorted_arrays::find_median_sorted_arrays(vec![], vec![2]),
      2.0
    );
  }
}

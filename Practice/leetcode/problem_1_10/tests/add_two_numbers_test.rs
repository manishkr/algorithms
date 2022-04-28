#[cfg(test)]
mod tests {
  use problem_1_10::add_two_numbers;
  use problem_1_10::add_two_numbers::ListNode;

  #[test]
  fn test_add_two_numbers_1() {
    assert_eq!(add_two_numbers::add_two_numbers(None::<Box<ListNode>>,
                                                None::<Box<ListNode>>), None::<Box<ListNode>>);
  }

  #[test]
  fn test_add_two_numbers_2() {
    let list = ListNode {
      val: 2,
      next: Some(Box::from(ListNode {
        val: 4,
        next: Some(Box::from(ListNode { val: 3, next: None })),
      })),
    };

    let expected_list = ListNode {
      val: 2,
      next: Some(Box::from(ListNode {
        val: 4,
        next: Some(Box::from(ListNode { val: 3, next: None })),
      })),
    };

    assert_eq!(add_two_numbers::add_two_numbers(Some(Box::from(list)),
                                                None::<Box<ListNode>>), Some(Box::from(expected_list)));
  }

  #[test]
  fn test_add_two_numbers_3() {
    let list = ListNode {
      val: 5,
      next: Some(Box::from(ListNode {
        val: 6,
        next: Some(Box::from(ListNode { val: 4, next: None })),
      })),
    };

    let expected_list = ListNode {
      val: 5,
      next: Some(Box::from(ListNode {
        val: 6,
        next: Some(Box::from(ListNode { val: 4, next: None })),
      })),
    };

    assert_eq!(add_two_numbers::add_two_numbers(None::<Box<ListNode>>,
                                                Some(Box::from(list))), Some(Box::from(expected_list)));
  }

  #[test]
  fn test_add_two_numbers_4() {
    let list_1 = ListNode {
      val: 9,
      next: None
    };

    let list_2 = ListNode {
      val: 8,
      next: None
    };

    let expected_list = ListNode {
      val: 7,
      next: Some(Box::from(ListNode {
        val: 1,
        next: None
      })),
    };

    assert_eq!(add_two_numbers::add_two_numbers(Some(Box::from(list_1)),
                                                Some(Box::from(list_2))), Some(Box::from(expected_list)));
  }

  #[test]
  fn test_add_two_numbers_5() {
    let list_1 = ListNode {
      val: 2,
      next: Some(Box::from(ListNode {
        val: 4,
        next: Some(Box::from(ListNode { val: 3, next: None })),
      })),
    };

    let list_2 = ListNode {
      val: 5,
      next: Some(Box::from(ListNode {
        val: 6,
        next: Some(Box::from(ListNode { val: 4, next: None })),
      })),
    };

    let expected_list = ListNode {
      val: 7,
      next: Some(Box::from(ListNode {
        val: 0,
        next: Some(Box::from(ListNode { val: 8, next: None })),
      })),
    };

    assert_eq!(add_two_numbers::add_two_numbers(Some(Box::from(list_1)),
                                                Some(Box::from(list_2))), Some(Box::from(expected_list)));
  }

  #[test]
  fn test_add_two_numbers_6() {
    let list_1 = ListNode {
      val: 0,
      next: None,
    };

    let list_2 = ListNode {
      val: 0,
      next: None,
    };

    let expected_list = ListNode {
      val: 0,
      next: None,
    };

    assert_eq!(add_two_numbers::add_two_numbers(Some(Box::from(list_1)),
                                                Some(Box::from(list_2))), Some(Box::from(expected_list)));
  }

  #[test]
  fn test_add_two_numbers_7() {
    let list_1 = ListNode {
      val: 9,
      next: Some(Box::from(ListNode {
        val: 9,
        next: Some(Box::from(ListNode {
          val: 9,
          next: Some(Box::from(ListNode {
            val: 9,
            next: Some(Box::from(ListNode {
              val: 9,
              next: Some(Box::from(ListNode {
                val: 9,
                next: Some(Box::from(ListNode {
                  val: 9,
                  next: None,
                })),
              })),
            })),
          })),
        })),
      })),
    };

    let list_2 = ListNode {
      val: 9,
      next: Some(Box::from(ListNode {
        val: 9,
        next: Some(Box::from(ListNode {
          val: 9,
          next: Some(Box::from(ListNode {
            val: 9,
            next: None,
          })),
        })),
      })),
    };

    let expected_list = ListNode {
      val: 8,
      next: Some(Box::from(ListNode {
        val: 9,
        next: Some(Box::from(ListNode {
          val: 9,
          next: Some(Box::from(ListNode {
            val: 9,
            next: Some(Box::from(ListNode {
              val: 0,
              next: Some(Box::from(ListNode {
                val: 0,
                next: Some(Box::from(ListNode {
                  val: 0,
                  next: Some(Box::from(ListNode {
                    val: 1,
                    next: None,
                  })),
                })),
              })),
            })),
          })),
        })),
      })),
    };

    assert_eq!(add_two_numbers::add_two_numbers(Some(Box::from(list_1)),
                                                Some(Box::from(list_2))), Some(Box::from(expected_list)));
  }
}

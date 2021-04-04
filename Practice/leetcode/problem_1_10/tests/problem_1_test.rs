#[cfg(test)]
mod tests {
    #[test]
    fn test_two_sum_1() {
        assert_eq!(problem_1::two_sum(vec![1, 2, 3], 5), vec![1, 2]);
    }

    #[test]
    fn test_two_sum_2() {
        assert_eq!(problem_1::two_sum(vec![1, 2, 3], 11), vec![]);
    }

    #[test]
    fn test_two_sum_3() {
        assert_eq!(problem_1::two_sum(vec![2,7,11,15], 9), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_4() {
        assert_eq!(problem_1::two_sum(vec![3,2,4], 6), vec![1, 2]);
    }

    #[test]
    fn test_two_sum_5() {
        assert_eq!(problem_1::two_sum(vec![3, 3], 6), vec![0,1]);
    }
}

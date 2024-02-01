#[cfg(test)]
mod tests {
    use problem_1_10::regular_expression_matching;

    #[test]
    fn test_regular_expression_matching_1() {
        assert_eq!(
            regular_expression_matching::is_match("a".to_owned(), "aa".to_owned()),
            false
        );
    }

    #[test]
    fn test_regular_expression_matching_2() {
        assert_eq!(
            regular_expression_matching::is_match("a*".to_owned(), "aa".to_owned()),
            true
        );
        assert_eq!(
            regular_expression_matching::is_match("a*".to_owned(), "aaa".to_owned()),
            true
        );
        assert_eq!(
            regular_expression_matching::is_match("a*".to_owned(), "aaaa".to_owned()),
            true
        );
    }

    #[test]
    fn test_regular_expression_matching_3() {
        assert_eq!(
            regular_expression_matching::is_match("p.".to_owned(), "pq".to_owned()),
            true
        );
    }

    #[test]
    fn test_regular_expression_matching_4() {
        assert_eq!(
            regular_expression_matching::is_match("zxqprrs".to_owned(), "zxqprrs".to_owned()),
            true
        );
    }

    #[test]
    fn test_regular_expression_matching_5() {
        assert_eq!(
            regular_expression_matching::is_match("c*a*b".to_owned(), "aab".to_owned()),
            true
        );
    }

    #[test]
    fn test_regular_expression_matching_6() {
        assert_eq!(
            regular_expression_matching::is_match("a.a".to_owned(), "aaa".to_owned()),
            true
        );
    }

    #[test]
    fn test_regular_expression_matching_7() {
        assert_eq!(
            regular_expression_matching::is_match("a*a".to_owned(), "a".to_owned()),
            true
        );
        assert_eq!(
            regular_expression_matching::is_match("a*a".to_owned(), "aaa".to_owned()),
            true
        );
        assert_eq!(
            regular_expression_matching::is_match("a*a".to_owned(), "aaaa".to_owned()),
            true
        );
    }

    #[test]
    fn test_regular_expression_matching_8() {
        assert_eq!(
            regular_expression_matching::is_match("ab*ac*a".to_owned(), "aaa".to_owned()),
            true
        );
    }

    #[test]
    fn test_regular_expression_matching_9() {
        assert_eq!(
            regular_expression_matching::is_match("a*a".to_owned(), "a".to_owned()),
            true
        );
    }

    #[test]
    fn test_regular_expression_matching_10() {
        assert_eq!(
            regular_expression_matching::is_match(".*".to_owned(), "ab".to_owned()),
            true
        );
    }
}

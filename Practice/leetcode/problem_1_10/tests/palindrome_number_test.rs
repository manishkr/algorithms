#[cfg(test)]
mod tests {
    use problem_1_10::palindrome_number;

    #[test]
    fn test_palindrome_number_1() {
        assert_eq!(palindrome_number::is_palindrome(121), true);
    }

    #[test]
    fn test_palindrome_number_2() {
        assert_eq!(palindrome_number::is_palindrome(123), false);
    }

    #[test]
    fn test_palindrome_number_3() {
        assert_eq!(palindrome_number::is_palindrome(-121), false);
    }

    #[test]
    fn test_palindrome_number_4() {
        assert_eq!(palindrome_number::is_palindrome(10), false);
    }

    #[test]
    fn test_palindrome_number_5() {
        assert_eq!(palindrome_number::is_palindrome(1), true);
    }
}

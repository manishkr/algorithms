#[cfg(test)]
mod tests {
    use problem_1_10::reverse_integer;

    #[test]
    fn test_reverse_integer_1() {
        assert_eq!(reverse_integer::reverse(0), 0);
    }

    #[test]
    fn test_reverse_integer_2() {
        assert_eq!(reverse_integer::reverse(123), 321);
    }

    #[test]
    fn test_reverse_integer_3() {
        assert_eq!(reverse_integer::reverse(-123), -321);
    }

    #[test]
    fn test_reverse_integer_4() {
        assert_eq!(reverse_integer::reverse(120), 21);
    }

    #[test]
    fn test_reverse_integer_5() {
        assert_eq!(reverse_integer::reverse(-11), -11);
    }

    #[test]
    fn test_reverse_integer_6() {
        assert_eq!(reverse_integer::reverse(1), 1);
    }

    #[test]
    fn test_reverse_integer_7() {
        assert_eq!(reverse_integer::reverse(100), 1);
    }

    #[test]
    fn test_reverse_integer_8() {
        assert_eq!(reverse_integer::reverse(1534236469), 0);
    }
}

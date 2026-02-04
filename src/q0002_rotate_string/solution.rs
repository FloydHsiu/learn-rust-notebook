pub fn rotate_string(s: &str, goal: &str) -> bool {
    // 1. The lengths must be equal. If not, it's impossible for one to be a rotation of the other.
    // 2. If lengths are equal, we can use a clever trick: if `goal` is a rotation of `s`,
    //    then `goal` must be a substring of `s` concatenated with itself.
    //    Example: s = "abcde", s+s = "abcdeabcde". The rotation "cdeab" is in there.
    // 3. The length check also handles an edge case for the `contains` method.
    //    For s = "ab", goal = "b", s+s = "abab". "abab".contains("b") is true, but it's not a rotation.
    //    The initial length check prevents this.
    s.len() == goal.len() && (s.to_owned() + s).contains(goal)
}

#[cfg(test)]
mod tests {
    use super::rotate_string;

    #[test]
    fn test_rotate_string_success() {
        assert_eq!(rotate_string("abcde", "cdeab"), true);
        assert_eq!(rotate_string("abcde", "abcde"), true); // 0 shifts
    }

    #[test]
    fn test_rotate_string_failure() {
        assert_eq!(rotate_string("abcde", "abced"), false);
        assert_eq!(rotate_string("abcde", "cedab"), false);
    }

    #[test]
    fn test_different_lengths() {
        assert_eq!(rotate_string("aa", "a"), false);
        assert_eq!(rotate_string("a", "aa"), false);
    }

    #[test]
    fn test_empty_strings() {
        assert_eq!(rotate_string("", ""), true);
        assert_eq!(rotate_string("a", ""), false);
        assert_eq!(rotate_string("", "a"), false);
    }
}
pub fn repeated_substring_pattern(s: String) -> bool {
    let n = s.len();
    if n < 2 {
        return false;
    }

    (1..=n / 2)
        .filter(|&len| n % len == 0)
        .any(|len| {
            let pattern = &s[..len];
            s.as_bytes()
                .chunks_exact(len)
                .all(|chunk| chunk == pattern.as_bytes())
        })
}

#[cfg(test)]
mod tests {
    use super::repeated_substring_pattern;

    #[test]
    fn test_repeated_substring_pattern() {
        assert_eq!(repeated_substring_pattern("abab".to_string()), true);
        assert_eq!(repeated_substring_pattern("aba".to_string()), false);
        assert_eq!(repeated_substring_pattern("abcabcabcabc".to_string()), true);
        assert_eq!(repeated_substring_pattern("a".to_string()), false);
        assert_eq!(repeated_substring_pattern("".to_string()), false);
        assert_eq!(repeated_substring_pattern("abac".to_string()), false);
    }
}
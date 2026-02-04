pub fn repeated_substring_pattern(s: String) -> bool {
    let n = s.len();
    let max_substr_len = s.len() / 2;

    for i in 1..max_substr_len+1 {
        if n % i != 0 {
            continue;
        }

        let mut is_repeated = true;
        for j in 1..(n/i) {
            let start_idx = j*i;
            let end_idx = start_idx + i;
            if s[0..i] != s[start_idx..end_idx] {
                is_repeated = false;
                break;
            }
        }
        if is_repeated {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::repeated_substring_pattern;

    #[test]
    fn it_works() {
        assert_eq!(repeated_substring_pattern("abab".to_string()), true);
        assert_eq!(repeated_substring_pattern("aba".to_string()), false);
        assert_eq!(repeated_substring_pattern("abcabcabcabc".to_string()), true);
        assert_eq!(repeated_substring_pattern("a".to_string()), false);
        assert_eq!(repeated_substring_pattern("".to_string()), false);
        assert_eq!(repeated_substring_pattern("abac".to_string()), false);
    }
}
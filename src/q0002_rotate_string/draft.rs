pub fn rotate_string(s: String, goal: String) -> bool {
    let s_concat = format!("{}{}", &s, &s);
    s_concat.contains(&goal) && (s.len() == goal.len())
}

#[cfg(test)]
mod tests {
    use super::rotate_string;

    #[test]
    fn it_works() {
        assert_eq!(rotate_string("aa".to_string(), "a".to_string()), false);
        assert_eq!(rotate_string("abcde".to_string(), "cdeab".to_string()), true);
        assert_eq!(rotate_string("abcde".to_string(), "abced".to_string()), false);
        assert_eq!(rotate_string("abcde".to_string(), "cedab".to_string()), false);
    }
}
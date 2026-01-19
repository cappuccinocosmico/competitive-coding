pub fn longest_substring_no_repition(s: &str) -> &str {
    let mut tmp_charset = std::collections::HashMap::new();
    let mut candidate_return = &s[0..0];
    let mut window_start = 0;
    let mut window_end = 0;
    for (index, char) in s.char_indices() {
        let charlen = char.len_utf8();
        if let Some(pos) = tmp_charset.get(&char) {
            if window_end - window_start > candidate_return.len() {
                candidate_return = &s[window_start..window_end];
            }
            window_start = window_start.max(pos + charlen);
            window_end = index + charlen;
            tmp_charset.insert(char, index);
        } else {
            tmp_charset.insert(char, index);
            window_end = index + charlen;
        };
    }

    if window_end - window_start > candidate_return.len() {
        candidate_return = &s[window_start..window_end];
    }

    candidate_return
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(longest_substring_no_repition(""), "");
    }

    #[test]
    fn test_single_whitespace() {
        assert_eq!(longest_substring_no_repition(" "), " ");
    }

    #[test]
    fn test_single_character() {
        assert_eq!(longest_substring_no_repition("a"), "a");
    }

    #[test]
    fn test_all_unique_characters() {
        assert_eq!(longest_substring_no_repition("abcdef"), "abcdef");
    }

    #[test]
    fn test_all_same_characters() {
        assert_eq!(longest_substring_no_repition("aaaaaa"), "a");
    }

    #[test]
    fn test_repeated_pattern() {
        assert_eq!(longest_substring_no_repition("abcabcbb"), "abc");
    }

    #[test]
    fn test_complex_case() {
        assert_eq!(longest_substring_no_repition("pwwkew"), "wke");
    }

    #[test]
    fn test_longer_valid_substring_after_duplicate() {
        assert_eq!(longest_substring_no_repition("dvdf"), "vdf");
    }

    #[test]
    fn test_other_stuff() {
        assert_eq!(longest_substring_no_repition("abbba"), "ab");
    }

    #[test]
    fn test_unicode_characters() {
        assert_eq!(longest_substring_no_repition("a测试b测c试d"), "b测c试d");
    }

    #[test]
    fn test_mixed_ascii_and_unicode() {
        assert_eq!(longest_substring_no_repition("ab中cde中f"), "ab中cde");
    }
}

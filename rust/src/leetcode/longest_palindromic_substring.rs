struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        longest_palindrome_str_slow(&s).to_string()
    }
}

fn is_string_byte_palindrome(s: &str) -> bool {
    for i in 0..s.len() / 2 {
        if s.as_bytes()[i] != s.as_bytes()[s.len() - 1 - i] {
            return false;
        }
    }
    true
}

fn check_all_substrings_for_longest(s: &str, eval: impl Fn(&str) -> bool) -> &str {
    for substring_len in (0..s.len()).rev() {
        for starting_index in 0..(s.len() - substring_len) {
            let substring = &s[starting_index..starting_index + substring_len + 1];
            if eval(substring) {
                return substring;
            }
            println!("Substring did not meet eval: {substring}")
        }
    }
    ""
}

fn longest_palindrome_str_slow(s: &str) -> &str {
    check_all_substrings_for_longest(s, is_string_byte_palindrome)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let result = Solution::longest_palindrome("babad".to_string());
        assert!(
            result == "bab" || result == "aba",
            "Expected 'bab' or 'aba', but got '{result}'",
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    }

    #[test]
    fn test_single_char() {
        let result = Solution::longest_palindrome("a".to_string());
        assert_eq!(result, "a");
    }

    #[test]
    fn test_empty_string_panics() {
        Solution::longest_palindrome("".to_string());
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::longest_palindrome("aaaa".to_string()), "aaaa");
    }

    #[test]
    fn test_no_palindrome() {
        let result = Solution::longest_palindrome("abcde".to_string());
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_long_odd_palindrome() {
        let result = Solution::longest_palindrome("racecar".to_string());
        assert_eq!(result, "racecar");
    }

    #[test]
    fn test_long_even_palindrome() {
        assert_eq!(Solution::longest_palindrome("hannah".to_string()), "hannah");
    }
}

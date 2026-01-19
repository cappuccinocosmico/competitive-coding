// You are given a string s and an array of strings words. All the strings of words are of the same length.
//
// A concatenated string is a string that exactly contains all the strings of any permutation of words concatenated.
//
//     For example, if words = ["ab","cd","ef"], then "abcdef", "abefcd", "cdabef", "cdefab", "efabcd", and "efcdab" are all concatenated strings. "acdbef" is not a concatenated string because it is not the concatenation of any permutation of words.
//
// Return an array of the starting indices of all the concatenated substrings in s. You can return the answer in any order.
struct Solution {}
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let vec_index_builder: Vec<(usize, Option<usize>)> = vec![];
        let mut examined_index = 0;
        while examined_index < s.len() {
            let check_substr = &s[examined_index..];
            for check_str in &words {
                let does_begin = "";
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "barfoothefoobarman".to_string();
        let words = vec!["foo".to_string(), "bar".to_string()];
        let mut result = Solution::find_substring(s, words);
        result.sort();
        assert_eq!(result, vec![0, 9]);
    }

    #[test]
    fn test_example_2() {
        let s = "wordgoodgoodgoodbestword".to_string();
        let words = vec![
            "word".to_string(),
            "good".to_string(),
            "best".to_string(),
            "word".to_string(),
        ];
        assert_eq!(Solution::find_substring(s, words), vec![]);
    }

    #[test]
    fn test_example_3() {
        let s = "barfoofoobarthefoobarman".to_string();
        let words = vec!["bar".to_string(), "foo".to_string(), "the".to_string()];
        let mut result = Solution::find_substring(s, words);
        result.sort();
        assert_eq!(result, vec![6, 9, 12]);
    }
}

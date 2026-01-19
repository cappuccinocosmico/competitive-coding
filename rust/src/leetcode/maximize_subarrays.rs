struct Solution {}
impl Solution {
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        let mut max_count = 0;

        for removed_index in 0..conflicting_pairs.len() {
            let mut count = 0;

            for start in 1..=n {
                for end in start..=n {
                    let mut is_valid = true;

                    for (pair_index, pair) in conflicting_pairs.iter().enumerate() {
                        if pair_index != removed_index {
                            let a = pair[0];
                            let b = pair[1];

                            if start <= a && a <= end && start <= b && b <= end {
                                is_valid = false;
                                break;
                            }
                        }
                    }

                    if is_valid {
                        count += 1;
                    }
                }
            }

            max_count = max_count.max(count);
        }

        max_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 4;
        let conflicting_pairs = vec![vec![2, 3], vec![1, 4]];
        let result = Solution::max_subarrays(n, conflicting_pairs);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_example_2() {
        let n = 5;
        let conflicting_pairs = vec![vec![1, 2], vec![2, 5], vec![3, 5]];
        let result = Solution::max_subarrays(n, conflicting_pairs);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_example_3() {
        let n = 10;
        let conflicting_pairs = vec![vec![10, 5], vec![3, 8]];
        let result = Solution::max_subarrays(n, conflicting_pairs);
        assert_eq!(result, 50);
    }
}

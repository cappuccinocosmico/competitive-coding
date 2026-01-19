// 2210. Count Hills and Valleys in an Array
// Easy
// Topics
// premium lock iconCompanies
// Hint
//
// You are given a 0-indexed integer array nums. An index i is part of a hill in nums if the closest non-equal neighbors of i are smaller than nums[i]. Similarly, an index i is part of a valley in nums if the closest non-equal neighbors of i are larger than nums[i]. Adjacent indices i and j are part of the same hill or valley if nums[i] == nums[j].
//
// Note that for an index to be part of a hill or valley, it must have a non-equal neighbor on both the left and right of the index.
//
// Return the number of hills and valleys in nums.
//
//
//
struct Solution {}
impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut inflection_count = 0;
        if nums.len() <= 2 {
            return inflection_count;
        }
        let mut previous_inflection = None;
        for index in 1..nums.len() {
            let (prev_num, current_num) = (nums[index - 1], nums[index]);
            if prev_num != current_num {
                let sign = prev_num < current_num;
                if let Some(prev_infl) = previous_inflection
                    && prev_infl != sign
                {
                    inflection_count += 1;
                }
                previous_inflection = Some(sign);
            }
        }
        inflection_count
    }
}

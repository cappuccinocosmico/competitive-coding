// You are given two arrays of integers, fruits and baskets, each of length n, where fruits[i] represents the quantity of the ith type of fruit, and baskets[j] represents the capacity of the jth basket.
//
// From left to right, place the fruits according to these rules:
//
//     Each fruit type must be placed in the leftmost available basket with a capacity greater than or equal to the quantity of that fruit type.
//     Each basket can hold only one type of fruit.
//     If a fruit type cannot be placed in any basket, it remains unplaced.
//
// Return the number of fruit types that remain unplaced after all possible allocations are made.
struct Solution {}
impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let answer_slow = fruit_baskets_slow(fruits.clone(), baskets.clone());
        let answer_fast = fruit_baskets_fast(fruits, baskets);
        answer_slow
        // assert!(
        //     answer_slow == answer_fast,
        //     "Results from different implementations were not identical"
        // );
        // answer_fast
    }
}

pub fn fruit_baskets_slow(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
    let mut unplaced_fruits = 0;
    let mut is_bucket_free = vec![true; baskets.len()];
    for fruit_amount in fruits {
        let mut is_fruit_unplaced = true;
        for (basket_index, basket_capacity) in baskets.iter().enumerate() {
            if *basket_capacity >= fruit_amount && is_bucket_free[basket_index] {
                is_bucket_free[basket_index] = false;
                is_fruit_unplaced = false;
                break;
            }
        }
        unplaced_fruits += is_fruit_unplaced as i32;
    }
    unplaced_fruits
}

pub fn fruit_baskets_fast(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
    0
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let fruits = vec![4, 2, 5];
        let baskets = vec![3, 5, 4];
        assert_eq!(Solution::num_of_unplaced_fruits(fruits, baskets), 1);
    }

    #[test]
    fn test_example_2() {
        let fruits = vec![3, 6, 1];
        let baskets = vec![6, 4, 7];
        assert_eq!(Solution::num_of_unplaced_fruits(fruits, baskets), 0);
    }

    #[test]
    fn test_empty_inputs() {
        let fruits = vec![];
        let baskets = vec![];
        assert_eq!(Solution::num_of_unplaced_fruits(fruits, baskets), 0);
    }

    #[test]
    fn test_more_fruits_than_baskets() {
        let fruits = vec![1, 1, 1];
        let baskets = vec![1, 1];
        assert_eq!(Solution::num_of_unplaced_fruits(fruits, baskets), 1);
    }

    #[test]
    fn test_no_suitable_baskets() {
        let fruits = vec![5, 5, 5];
        let baskets = vec![4, 4, 4];
        assert_eq!(Solution::num_of_unplaced_fruits(fruits, baskets), 3);
    }
}


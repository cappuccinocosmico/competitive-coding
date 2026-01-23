use std::collections::BTreeMap;

// The Elves in the kitchen explain the situation: because of their complicated new inventory management system, they can't figure out which of their ingredients are fresh and which are spoiled. When you ask how it works, they give you a copy of their database (your puzzle input).
//
// The database operates on ingredient IDs. It consists of a list of fresh ingredient ID ranges, a blank line, and a list of available ingredient IDs. For example:
//
const EXAMPLE_INPUT: &str = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32";
//
// The fresh ID ranges are inclusive: the range 3-5 means that ingredient IDs 3, 4, and 5 are all fresh. The ranges can also overlap; an ingredient ID is fresh if it is in any range.
//
// The Elves are trying to determine which of the available ingredient IDs are fresh. In this example, this is done as follows:
//
//     Ingredient ID 1 is spoiled because it does not fall into any range.
//     Ingredient ID 5 is fresh because it falls into range 3-5.
//     Ingredient ID 8 is spoiled.
//     Ingredient ID 11 is fresh because it falls into range 10-14.
//     Ingredient ID 17 is fresh because it falls into range 16-20 as well as range 12-18.
//     Ingredient ID 32 is spoiled.
//
// So, in this example, 3 of the available ingredient IDs are fresh.
//
// Process the database file from the new inventory management system. How many of the available ingredient IDs are fresh?

use competitive_coding::inputs::ADVENT_INPUT_DAY_5;

#[derive(Debug, Clone, Copy)]
struct IdRange {
    lower: u64,
    upper: u64,
}

impl IdRange {
    fn new(lower: u64, upper: u64) -> Option<Self> {
        if lower <= upper {
            Some(IdRange { lower, upper })
        } else {
            None
        }
    }
    // fn sum_all_values_in_range(&self) -> u64 {
    //     // Yay science
    //     ((self.lower + self.upper) * (self.upper + 1 - self.lower)) / 2
    // }
    fn count_all_values_in_range(&self) -> u64 {
        // Yay science
        self.upper + 1 - self.lower
    }
}

struct OrderedIntervalSet(BTreeMap<u64, IdRange>);

impl OrderedIntervalSet {
    const fn new() -> Self {
        OrderedIntervalSet(BTreeMap::new())
    }
    const fn get_underlying_map(&self) -> &BTreeMap<u64, IdRange> {
        &self.0
    }
    fn insert_interval(&mut self, interval: IdRange) {
        let tree = &mut self.0;
        let lower_interval_values_to_get_replaced = tree
            .range(interval.lower..=interval.upper)
            .map(|x| *x.0)
            .collect::<Vec<_>>();
        let mut upper_bound = lower_interval_values_to_get_replaced
            .into_iter()
            .map(|val| {
                tree.remove(&val)
                    .expect("All of these values should be in the tree")
                    .upper
            })
            .max()
            .map_or(interval.upper, |val| val.max(interval.upper));
        let low_key_optional = tree.range(..=interval.lower).next_back().and_then(|x| {
            if x.1.upper >= interval.lower {
                Some(*x.0)
            } else {
                None
            }
        });
        let lower_bound = low_key_optional
            .and_then(|low_key| {
                tree.remove(&low_key).map(|val| {
                    upper_bound = upper_bound.max(val.upper);
                    val.lower.min(interval.lower)
                })
            })
            .unwrap_or(interval.lower);
        let new_interval = IdRange {
            lower: lower_bound,
            upper: upper_bound,
        };
        let _ = tree.insert(new_interval.lower, new_interval);
    }

    fn from_interval_iterator(inputs: impl Iterator<Item = IdRange>) -> Self {
        let mut new_set = Self::new();
        for input in inputs {
            new_set.insert_interval(input);
        }
        new_set
    }

    fn is_item_in(&self, item: &u64) -> bool {
        if let Some(greatest_lower_interval) = self.0.range(..=*item).next_back().map(|x| *x.1) {
            *item <= greatest_lower_interval.upper
        } else {
            false
        }
    }
}

fn parse_inputs(raw: &str) -> (impl Iterator<Item = IdRange>, impl Iterator<Item = u64>) {
    let mut parts = raw.split("\n\n");
    let interval_part = parts.next().unwrap();
    let number_part = parts.next().unwrap();
    let intervals_iterator = interval_part.split_whitespace().map(|interval_str| {
        let mut val = interval_str.split("-");
        let lower = val.next().unwrap().trim().parse::<u64>().unwrap();
        let upper = val.next().unwrap().trim().parse::<u64>().unwrap();
        IdRange::new(lower, upper).unwrap()
    });
    let numbers_iterator = number_part
        .split_whitespace()
        .map(|number_str| number_str.trim().parse::<u64>().unwrap());
    (intervals_iterator, numbers_iterator)
}

fn run_task_1(raw: &str) -> u64 {
    let (intervals, numbers) = parse_inputs(raw);
    let interval_set = OrderedIntervalSet::from_interval_iterator(intervals);
    numbers
        .map(|num| interval_set.is_item_in(&num) as u64)
        .sum()
}

fn run_task_2(raw: &str) -> u64 {
    let (intervals, _) = parse_inputs(raw);
    let interval_set = OrderedIntervalSet::from_interval_iterator(intervals);
    let count_all_values = interval_set
        .get_underlying_map()
        .iter()
        .map(|x| x.1.count_all_values_in_range())
        .sum();
    count_all_values
}
//     Ingredient ID 1 is spoiled because it does not fall into any range.
//     Ingredient ID 5 is fresh because it falls into range 3-5.
//     Ingredient ID 8 is spoiled.
//     Ingredient ID 11 is fresh because it falls into range 10-14.
//     Ingredient ID 17 is fresh because it falls into range 16-20 as well as range 12-18.
//     Ingredient ID 32 is spoiled.
fn run_example_task_1() {
    let (intervals, numbers) = parse_inputs(EXAMPLE_INPUT);
    let fresh_set = OrderedIntervalSet::from_interval_iterator(intervals);
    assert!(!fresh_set.is_item_in(&1));
    assert!(fresh_set.is_item_in(&5));
    assert!(!fresh_set.is_item_in(&8));
    assert!(fresh_set.is_item_in(&11));
    assert!(fresh_set.is_item_in(&17));
    assert!(!fresh_set.is_item_in(&32));
}
fn run_example_task_2() {
    let example_val = run_task_2(EXAMPLE_INPUT);
    assert_eq!(example_val, 14)
}

fn main() {
    run_example_task_1();
    run_example_task_2();
    // 591 low.
    println!("Output 1:{}", run_task_1(ADVENT_INPUT_DAY_5));
    println!("Output 2:{}", run_task_2(ADVENT_INPUT_DAY_5))
}

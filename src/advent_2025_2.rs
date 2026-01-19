// They've even checked most of the product ID ranges already; they only have a few product ID ranges (your puzzle input) that you'll need to check. For example:
//
// 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
// 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
// 824824821-824824827,2121212118-2121212124
//
// (The ID ranges are wrapped here for legibility; in your input, they appear on a single long line.)
//
// The ranges are separated by commas (,); each range gives its first ID and last ID separated by a dash (-).
//
// Since the young Elf was just doing silly patterns, you can find the invalid IDs by looking for any ID which is made only of some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.
//
// None of the numbers have leading zeroes; 0101 isn't an ID at all. (101 is a valid ID that you would ignore.)
//
// Your job is to find all of the invalid IDs that appear in the given ranges. In the above example:
//
//     11-22 has two invalid IDs, 11 and 22.
//     95-115 has one invalid ID, 99.
//     998-1012 has one invalid ID, 1010.
//     1188511880-1188511890 has one invalid ID, 1188511885.
//     222220-222224 has one invalid ID, 222222.
//     1698522-1698528 contains no invalid IDs.
//     446443-446449 has one invalid ID, 446446.
//     38593856-38593862 has one invalid ID, 38593859.
//     The rest of the ranges contain no invalid IDs.
//
// Adding up all the invalid IDs in this example produces 1227775554.

use std::ops::{Range, RangeInclusive};

// Idea for optimization, the sequence of all invalid id's is monotonic and increasing, because there is
// a cannonical bijection f(n) = ((n.to_string)+(n.to_string)).parse(), that takes every id and
// turns it into an invalid id.
//

fn invalid_ids_in_range(range: IdRange, valid_id_test: fn(u64) -> bool) -> Vec<u64> {
    let mut return_vec = Vec::new();
    let real_range: RangeInclusive<u64> = range.into();
    for id in real_range {
        if !valid_id_test(id) {
            return_vec.push(id)
        }
    }
    return_vec
}

#[derive(Clone, Copy)]
struct IdRange {
    start: u64,
    end: u64,
}

impl From<IdRange> for RangeInclusive<u64> {
    fn from(value: IdRange) -> Self {
        value.start..=value.end
    }
}

fn parse_input_into_ranges(inputs: &str) -> Vec<IdRange> {
    let comma_seperated_values = inputs.trim().split(',');
    fn generate_range_single(input: &str) -> IdRange {
        let values: Vec<&str> = input.trim().split('-').collect();
        let start = values[0]
            .trim()
            .parse()
            .expect("Input 1 should have been a parsable int");
        let end = values[1]
            .trim()
            .parse()
            .expect("Input 2 should have been a parsable int");
        IdRange { start, end }
    }
    comma_seperated_values.map(generate_range_single).collect()
}

fn add_all_invalid_ids_together(inputs: &str, valid_id_test: fn(u64) -> bool) -> u64 {
    let ranges = parse_input_into_ranges(inputs);
    let invalid_ids_sum = ranges
        .iter()
        .copied()
        .map(|x| invalid_ids_in_range(x, valid_id_test))
        .map(|vector: Vec<u64>| vector.into_iter().sum::<u64>())
        .sum();
    invalid_ids_sum
}
fn is_id_valid_1(id: u64) -> bool {
    let digit_characters: Vec<char> = id.to_string().chars().collect();
    if digit_characters.len() % 2 != 0 {
        return true;
    };
    let substring_len = digit_characters.len() / 2;
    for i in 0..substring_len {
        if digit_characters[i] != digit_characters[i + substring_len] {
            return true;
        }
    }
    false
}

const SMALL_PRIMES: &[usize] = &[2, 3, 5, 7, 11, 13, 17, 19];
fn is_factor_of_prime(num: usize, prime: usize) -> bool {
    num % prime == 0 && prime <= num
}

fn is_id_valid_2(id: u64) -> bool {
    // Checks any multiple so its okay to only actually check prime multiples, since a repetiton
    // factor of 6 also shows up as a repetion factor of 2 and 3.
    let digit_characters: Vec<char> = id.to_string().chars().collect();
    for prime in SMALL_PRIMES {
        if !id_valid_for_prime(&digit_characters, *prime) {
            return false;
        }
    }
    return true;
    fn id_valid_for_prime(id: &[char], prime: usize) -> bool {
        if !is_factor_of_prime(id.len(), prime) {
            return true;
        };
        let substring_len = id.len() / prime;
        for i in 0..substring_len {
            for m in 1..prime {
                if id[i] != id[i + m * substring_len] {
                    return true;
                }
            }
        }
        false
    }
}
fn example_input_produces_expected_sums() {
    let input = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";

    // Original rule: repeated exactly twice
    let result1 = add_all_invalid_ids_together(input, is_id_valid_1);
    assert_eq!(result1, 1_227_775_554);

    // New rule: repeated at least twice
    let result2 = add_all_invalid_ids_together(input, is_id_valid_2);
    assert_eq!(result2, 4_174_379_265);
}
fn main() {
    println!(
        "Output 1: {}",
        add_all_invalid_ids_together(INPUT, is_id_valid_1)
    );
    println!(
        "Output 2: {}",
        add_all_invalid_ids_together(INPUT, is_id_valid_2)
    );
    example_input_produces_expected_sums();
}

const INPUT: &str = "328412-412772,1610-2974,163-270,7693600637-7693779967,352-586,65728-111612,734895-926350,68-130,183511-264058,8181752851-8181892713,32291-63049,6658-12472,720-1326,21836182-21869091,983931-1016370,467936-607122,31-48,6549987-6603447,8282771161-8282886238,7659673-7828029,2-18,7549306131-7549468715,3177-5305,20522-31608,763697750-763835073,5252512393-5252544612,6622957-6731483,9786096-9876355,53488585-53570896";

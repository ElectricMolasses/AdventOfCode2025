use std::collections::HashSet;

use crate::util::read_by_line;

pub fn run_challenge_05_00(input_name: &str) -> u64 {
    let input = read_by_line(&("./src/challenges/challenge_05/".to_owned() + input_name));

    let mut fresh_list: Vec<(u64, u64)> = Vec::new();
    let mut available_list: Vec<u64> = Vec::new();

    let mut fresh_complete: bool = false;
    for line in input.lines() {
        if line.is_empty() {
            fresh_complete = true;
            continue;
        }
        if !fresh_complete {
            let items: Vec<u64> = line.split("-")
                                      .map(|x| x.parse::<u64>().unwrap())
                                      .collect();
            fresh_list.push((items[0], items[1]));
            continue;
        }
        available_list.push(line.parse::<u64>().unwrap());
    }

    let mut fresh_count: u64 = 0;

    for item in available_list {
        for range in &fresh_list {
            if item > range.0 && item <= range.1 {
                fresh_count += 1;
                break;
            }
        }
    }

    fresh_count
}

fn merge_ranges(ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut sorted_ranges: Vec<(u64, u64)> = ranges.clone();
    sorted_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    merged_ranges.push(sorted_ranges[0]);

    for range in sorted_ranges {
        let last_merged = merged_ranges.last().unwrap().clone();

        if range.0 <= last_merged.1 && range.1 <= last_merged.1 {
            continue;
        }
        if range.0 <= last_merged.1 && range.1 > last_merged.1 {
            merged_ranges.pop();
            merged_ranges.push((last_merged.0, range.1));
            continue;
        }
        merged_ranges.push(range);
    }

    merged_ranges
}

pub fn run_challenge_05_01(input_name: &str) -> u64 {
    let input = read_by_line(&("./src/challenges/challenge_05/".to_owned() + input_name));

    let mut fresh_list: Vec<(u64, u64)> = Vec::new();
    let mut available_list: Vec<u64> = Vec::new();

    let mut fresh_complete: bool = false;
    for line in input.lines() {
        if line.is_empty() {
            fresh_complete = true;
            continue;
        }
        if !fresh_complete {
            let items: Vec<u64> = line.split("-")
                                      .map(|x| x.parse::<u64>().unwrap())
                                      .collect();
            fresh_list.push((items[0], items[1]));
            continue;
        }
        available_list.push(line.parse::<u64>().unwrap());
    }

    let merged_ranges = merge_ranges(fresh_list);

    let mut valid_ids: u64 = 0;

    for range in merged_ranges {
        valid_ids += range.1 - range.0 + 1;
    }

    valid_ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_01_sample() {
        assert_eq!(run_challenge_05_00("input_sample_a"), 3);
    }

    #[test]
    fn challenge_02_sample() {
        assert_eq!(run_challenge_05_01("input_sample_a"), 14);
    }
}

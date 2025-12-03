use crate::util::read_by_line;

fn find_largest_joltage(bank: &str) -> i64 {
    let batteries: Vec<u32> = bank.chars().map(|x| x.to_digit(10).unwrap()).collect();

    let mut first_idx = 0;
    for (idx, battery) in batteries.iter().enumerate() {
        if idx == batteries.len() - 1 { break };
        if *battery > batteries[first_idx] { first_idx = idx };
    }

    let mut second_idx = first_idx + 1;
    for idx in first_idx+1..batteries.len() {
        if batteries[idx] > batteries[second_idx] {
            second_idx = idx;
        }
    }

    (batteries[first_idx] * 10 + batteries[second_idx]) as i64
}

pub fn run_challenge_03_00(input_name: &str) -> i64 {
    let input = read_by_line(&("./src/challenges/challenge_03/".to_owned() + input_name));

    let mut banks: Vec<&str> = input.split("\n").map(|line| line.trim()).collect();
    banks.pop();

    let mut joltages: Vec<i64> = Vec::new();

    for bank in banks {
        joltages.push(find_largest_joltage(bank));
    }

    joltages.iter().sum()
}

fn find_largest_joltage_01(bank: &str) -> i64 {
    let batteries: Vec<u32> = bank.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let num_unused = bank.len() - 12;

    let mut current_start: usize = 0;
    let mut current_range: usize = num_unused+1;

    let mut batteries_on: Vec<u32> = Vec::new();

    while batteries_on.len() < 12 {
      let mut highest_idx: usize = current_start;

      for idx in current_start..current_start+current_range {
        if batteries[idx] > batteries[highest_idx] {
          highest_idx = idx;
        }
      }
      batteries_on.push(batteries[highest_idx]);
      current_range = current_start+current_range - highest_idx;
      current_start = highest_idx+1;
    }

    batteries_on.iter().map(|x| x.to_string())
                       .collect::<Vec<String>>()
                       .join("")
                       .parse::<i64>()
                       .unwrap()
}

pub fn run_challenge_03_01(input_name: &str) -> i64 {
    let input = read_by_line(&("./src/challenges/challenge_03/".to_owned() + input_name));

    let mut banks: Vec<&str> = input.split("\n").map(|line| line.trim()).collect();
    banks.pop();

    let mut joltages: Vec<i64> = Vec::new();

    for bank in banks {
        joltages.push(find_largest_joltage_01(bank));
    }

    joltages.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_01_sample() {
        assert_eq!(run_challenge_03_00("input_sample_a"), 357);
    }

    #[test]
    fn challenge_02_sample() {
        assert_eq!(run_challenge_03_01("input_sample_a"), 3121910778619);
    }

    #[test]
    fn test_find_largest_jolt_01_987654321111111() {
      assert_eq!(find_largest_joltage_01("987654321111111"), 987654321111);
    }

    #[test]
    fn test_find_largest_jolt_01_811111111111119() {
      assert_eq!(find_largest_joltage_01("811111111111119"), 811111111119);
    }

    #[test]
    fn test_find_largest_jolt_01_234234234234278() {
      assert_eq!(find_largest_joltage_01("234234234234278"), 434234234278);
    }

    #[test]
    fn test_find_largest_jolt_818181911112111() {
      assert_eq!(find_largest_joltage_01("818181911112111"), 888911112111);
    }
}

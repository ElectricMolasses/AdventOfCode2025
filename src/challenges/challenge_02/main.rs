use crate::util::read_by_line;

fn cut_string_sized_segments(text: &str, size: usize) -> Vec<&str> {
    let mut split_string: Vec<&str> = Vec::new();

    for idx in (0..text.len()).step_by(size) {
        split_string.push(&text[idx..idx+size]);
    }

    split_string
}

pub fn run_challenge_02_00(input_name: &str) -> i64 {
    let input = read_by_line(&("./src/challenges/challenge_02/".to_owned() + input_name));

    let ids: Vec<&str> = input.split(",").map(|id| id.trim()).collect();

    let mut invalid_ids: Vec<i64> = Vec::new();

    for id in &ids {
        let id_pair = id.split("-").collect::<Vec<&str>>();
        let start = id_pair.get(0).unwrap().parse::<i64>().unwrap();
        let end = id_pair.get(1).unwrap().parse::<i64>().unwrap();

        for id in start..end+1 {
            let id_str = id.to_string();

            if id_str.len() % 2 != 0 {
                continue;
            }

            let split_string = cut_string_sized_segments(&id_str, id_str.len() / 2);

            if (split_string[0] == split_string[1]) {
                invalid_ids.push(id_str.parse::<i64>().unwrap());
            }
        }
    }

    invalid_ids.iter().sum()
}

pub fn run_challenge_02_01(input_name: &str) -> i64 {
    let input = read_by_line(&("./src/challenges/challenge_02/".to_owned() + input_name));

    let ids: Vec<&str> = input.split(",").map(|id| id.trim()).collect();

    let mut invalid_ids: Vec<i64> = Vec::new();

    for id in &ids {
        let id_pair = id.split("-").collect::<Vec<&str>>();
        let start = id_pair.get(0).unwrap().parse::<i64>().unwrap();
        let end = id_pair.get(1).unwrap().parse::<i64>().unwrap();

        for id in start..end+1 {
            let id_str = id.to_string();
            
            for sub_size in 1..(id_str.len()/2)+1 {
                let mut found: bool = false;

                if id_str.len() % sub_size != 0 {
                    continue;
                }

                let string_pieces: Vec<&str> = cut_string_sized_segments(&id_str, sub_size);
                for (idx, piece) in string_pieces.iter().enumerate() {
                    if **piece != *string_pieces[0] {
                        break;
                    }

                    if idx+1 == string_pieces.len() {
                        invalid_ids.push(id_str.parse::<i64>().unwrap());
                        found = true;
                    }
                }
                if found { break; }

            }
        }
    }

    invalid_ids.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_01_sample() {
        assert_eq!(run_challenge_02_00("input_sample_a"), 1227775554);
    }

    #[test]
    fn challenge_02_sample() {
        assert_eq!(run_challenge_02_01("input_sample_a"), 4174379265);
    }
}

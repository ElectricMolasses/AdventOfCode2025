use crate::util::read_by_line;

pub fn run_challenge_00() -> i32 {
    let input = read_by_line("./src/challenges/challenge_01/input");

    let split_input = input.split("\n");

    let mut dial: i32 = 50;
    let mut answer: i32 = 0;

    for line in split_input {
        let direction = match line.get(..1) {
            None => "YEW STOOPUD",
            Some(ch) => ch
        };

        if (direction == "YEW STOOPUD") {
            continue;
        }

        let steps = match line.get(1..) {
            None => "YEW DEFINITELY STOOPUD",
            Some(st) => st
        };

        if steps == "YEW DEFINITELY STOOPUD" {
            panic!("HOW I SO STOOPUD?!?!?!");
        }

        let steps_integer = match steps.parse::<i32>() {
            Ok(int) => int,
            Err(_) => panic!("WHY NO CONVERT TO THE NUMBER FORM, PUH-LEASE!!!"),
        };
 

        match direction {
            "L" => dial -= steps_integer,
            "R" => dial += steps_integer,
            _ => panic!(" WHAT DA FOOK "),
        }

        while dial < 0 {
            dial += 100;
        }

        while dial > 99 {
            dial -= 100;
        }

        println!("direction: {}", direction);
        println!("steps: {}", steps_integer);
        println!("dial position: {}", dial);

        if dial == 0 { answer += 1; }
    }

    answer
}

pub fn run_challenge_01() -> i32 {
    let input = read_by_line("./src/challenges/challenge_01/input");

    let split_input = input.split("\n");

    let mut dial: i32 = 50;
    let mut last_dial: i32 = 50;
    let mut answer: i32 = 0;

    for line in split_input {
        let direction = match line.get(..1) {
            None => "YEW STOOPUD",
            Some(ch) => ch
        };

        if (direction == "YEW STOOPUD") {
            continue;
        }

        let steps = match line.get(1..) {
            None => "YEW DEFINITELY STOOPUD",
            Some(st) => st
        };

        if steps == "YEW DEFINITELY STOOPUD" {
            panic!("HOW I SO STOOPUD?!?!?!");
        }

        let steps_integer = match steps.parse::<i32>() {
            Ok(int) => int,
            Err(_) => panic!("WHY NO CONVERT TO THE NUMBER FORM, PUH-LEASE!!!"),
        };

        last_dial = dial;

        match direction {
            "L" => dial -= steps_integer,
            "R" => dial += steps_integer,
            _ => panic!(" WHAT DA FOOK "),
        }

        if dial < 0 {
            if last_dial == 0 { dial += 100; }
            while dial < 0 {
                if dial != 0 { answer += 1; }
                dial += 100;
            }
        }
        if dial > 99 {
            while dial > 99 {
                if dial != 100 { answer += 1; }
                dial -= 100;
            }
        }
        if dial == 0 { answer += 1; }


        println!("The dial is rotated {}{} to point at {}; The dial has touched zero {} time(s)", direction, steps_integer, dial, answer);
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(1, 1);
    }
}

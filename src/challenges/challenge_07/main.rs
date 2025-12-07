use crate::util::read_by_line;

fn print_simulation(input: &Vec<Vec<char>>) {
    for line in input.iter() {
        for c in line.iter() {
            print!("{}", c);
        }
        println!();
    }
}

fn count_char_in_simulation(input: &Vec<Vec<char>>, ic: char) -> u64 {
    let mut sum: u64 = 0;
    
    for line in input.iter() {
        for c in line.iter() {
            if *c == ic { sum += 1 }
        }
    }

    sum
}

fn read_into_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        grid.push(Vec::new());
        for c in line.chars() {
            grid[idx].push(c);
        }
    }

    grid
}

#[derive(Clone)]
struct BeamInstruction {
    x: usize,
    y: usize,
    character: char,
}

fn create_beam(simulation: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<BeamInstruction> {
    match simulation[y][x] {
        '.' => Vec::from([BeamInstruction{ x, y, character: '|' }]),
        '^' => Vec::from([
                create_beam(simulation, x-1, y),
                create_beam(simulation, x+1, y)
            ]).iter().flat_map(|inner_vec| inner_vec.iter()).cloned().collect(),
        '|' => Vec::new(),
        c => panic!("WEE-OO-EE-OO::Unhandle-able character! O.o -- {}", c),
    }
}

fn create_beam_did_split(simulation: &Vec<Vec<char>>, x: usize, y: usize) -> (Vec<BeamInstruction>, bool) {
    (match simulation[y][x] {
        '.' => Vec::from([BeamInstruction{ x, y, character: '|' }]),
        '^' => Vec::from([
                create_beam(simulation, x-1, y),
                create_beam(simulation, x+1, y)
            ]).iter().flat_map(|inner_vec| inner_vec.iter()).cloned().collect(),
        '|' => Vec::new(),
        c => panic!("WEE-OO-EE-OO::Unhandle-able character! O.o -- {}", c),
    },
    match simulation[y][x] {
        '^' => true,
        _ => false,
    })
}

fn simulate_beam(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut simulation = grid.clone();

for r_idx in 0..simulation.len()-1 {
        for c_idx in 0..simulation[r_idx].len() {
            let instructions: Vec<BeamInstruction> = match simulation[r_idx][c_idx] {
                'S' => {
                    create_beam(&simulation, c_idx, r_idx+1)
                },
                '|' => {
                    create_beam(&simulation, c_idx, r_idx+1)
                },
                '.' => {
                    Vec::new()
                },
                '^' => {
                    Vec::new()
                },
                c => panic!("WEE-OO-EE-OO::Unhandle-able character! O.o -- {}", c),
            };
            for BeamInstruction { x, y, character } in instructions {
                simulation[y][x] = character;
            }
        }
    }

    simulation
}

fn simulate_beam_count_splits(grid: &Vec<Vec<char>>) -> (Vec<Vec<char>>, u64) {
    let mut simulation = grid.clone();
    let mut splits: u64 = 0;

    for r_idx in 0..simulation.len()-1 {
        for c_idx in 0..simulation[r_idx].len() {
            let (instructions, did_split) = match simulation[r_idx][c_idx] {
                'S' => {
                    create_beam_did_split(&simulation, c_idx, r_idx+1)
                },
                '|' => {
                    create_beam_did_split(&simulation, c_idx, r_idx+1)
                },
                '.' => {
                    (Vec::new(), false)
                },
                '^' => {
                    (Vec::new(), false)
                },
                c => panic!("WEE-OO-EE-OO::Unhandle-able character! O.o -- {}", c),
            };
            if did_split { splits += 1 }
            for BeamInstruction { x, y, character } in instructions {
                simulation[y][x] = character;
            }
        }
    }

    (simulation, splits)
}

pub fn run_challenge_07_00(input_name: &str) -> u64 {
    let input = read_by_line(&("./src/challenges/challenge_07/".to_owned() + input_name));

    let grid = read_into_grid(&input);
    let (solution, splits) = simulate_beam_count_splits(&grid);

    splits
}

#[derive(Debug, Clone)]
enum GridItem {
    START,
    SPLIT,
    EMPTY,
    BEAM(u64),
}

#[derive(Debug, Clone)]
struct QBeamInstruction {
    x: usize,
    y: usize,
    item: GridItem,
}

fn read_into_quantum_grid(input: &str) -> Vec<Vec<GridItem>> {
    let mut grid: Vec<Vec<GridItem>> = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        grid.push(Vec::new());
        for c in line.chars() {
            grid[idx].push(match c {
                'S' => GridItem::START,
                '^' => GridItem::SPLIT,
                '.' => GridItem::EMPTY,
                c => panic!("WEE-OO-EE-OO::Unhandle-able character! O.o -- {}", c),
            });
        }
    }

    grid
}

fn print_quantum_simulation(input: &Vec<Vec<GridItem>>) {
    for line in input.iter() {
        for item in line.iter() {
            print!("{}", match item {
                GridItem::START => 'S',
                GridItem::SPLIT => '^',
                GridItem::EMPTY => '.',
                GridItem::BEAM(_) => '|',
            });
        }
        println!();
    }
}

fn create_beam_quantum(simulation: &Vec<Vec<GridItem>>, x: usize, y: usize, timelines: u64) -> Vec<QBeamInstruction> {
    match simulation[y][x] {
        GridItem::START => panic!("We found another start? I cannot believe you've done this."),
        GridItem::BEAM(v) => Vec::from([QBeamInstruction { x, y, item: GridItem::BEAM(v+timelines) }]),
        GridItem::EMPTY => Vec::from([QBeamInstruction { x, y, item: GridItem::BEAM(timelines) }]),
        GridItem::SPLIT => Vec::from([
            create_beam_quantum(simulation, x-1, y, timelines),
            create_beam_quantum(simulation, x+1, y, timelines),
        ]).iter().flat_map(|inner_vec| inner_vec.iter()).cloned().collect(),
    }
}

fn simulate_beam_count_splits_quantum(grid: &Vec<Vec<GridItem>>) -> Vec<Vec<GridItem>> {
    let mut simulation: Vec<Vec<GridItem>> = grid.iter().cloned().collect();

    for r_idx in 0..simulation.len()-1 {
        for c_idx in 0..simulation[r_idx].len() {
            let instructions = match simulation[r_idx][c_idx] {
                GridItem::START => create_beam_quantum(&simulation, c_idx, r_idx+1, 1),
                GridItem::BEAM(t) => create_beam_quantum(&simulation, c_idx, r_idx+1, t),
                GridItem::SPLIT => Vec::new(),
                GridItem::EMPTY => Vec::new(),
            };
            for QBeamInstruction { x, y, item } in instructions {
                simulation[y][x] = item;
            }
        }
    }

    simulation
}

fn count_quantum_timelines(grid: &Vec<Vec<GridItem>>) -> u64 {
    let mut timelines: u64 = 0;

    for item in grid.last().unwrap().iter() {
        if let GridItem::BEAM(t) = item {
            timelines += t;
        }
    }

    timelines
}

pub fn run_challenge_07_01(input_name: &str) -> u64 {
    let input = read_by_line(&("./src/challenges/challenge_07/".to_owned() + input_name));

    let grid = read_into_quantum_grid(&input);
    let solution = simulate_beam_count_splits_quantum(&grid);

    print_quantum_simulation(&solution);

    count_quantum_timelines(&solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_01_sample() {
        assert_eq!(run_challenge_07_00("input_sample_a"), 21);
    }

    #[test]
    fn challenge_02_sample() {
        assert_eq!(run_challenge_07_01("input_sample_a"), 40);
    }
}

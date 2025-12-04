use crate::util::read_by_line;

struct Coord {
    x: usize,
    y: usize,
}

struct Grid(Vec<Vec<char>>);

impl std::ops::Deref for Grid {
    type Target = Vec<Vec<char>>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl std::ops::DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> &char {
        return &self[y][x];
    }

    fn set(&mut self, x: usize, y: usize, to: char) {
        self[y][x] = to;
    }

    fn get_neighbours(&self, x_in: usize, y_in: usize) -> Vec<Coord> {
        let mut neighbours: Vec<Coord> = Vec::new();

        let x = x_in as i32;
        let y = y_in as i32;

        for y in y-1..y+2 {
            if y < 0 || y >= self.len() as i32 { continue }
            for x in x-1..x+2 {
                if x == x_in as i32 && y == y_in as i32 { continue }
                if x < 0 || x >= self[y as usize].len() as i32 { continue }

                if self[y as usize][x as usize] == '@' { 
                    neighbours.push(Coord {
                        x: x as usize,
                        y: y as usize,
                    }) 
                }
            }
        }

        neighbours
    }

    fn count_neighbours(&self, x_in: usize, y_in: usize) -> i32 {
        self.get_neighbours(x_in, y_in).len() as i32
    }

    fn get_items_with_n_or_fewer_neighbours(&self, n: i32) -> Vec<Coord> {
        let mut neighbours: Vec<Coord> = Vec::new();

        for (y, row) in self.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if *item == '@' {
                    let item_neighbours = self.count_neighbours(x, y);
                    if n > item_neighbours {
                        neighbours.push(Coord { x,y });
                    }
                }
            }
        }

        neighbours
    }

    fn sum_items_with_n_or_fewer_neighbours(&self, n: i32) -> i32 {
        self.get_items_with_n_or_fewer_neighbours(n).len() as i32
    }
}

fn build_array(input: &String) -> Grid {
   Grid(input.split("\n")
             .collect::<Vec<&str>>()
             .iter().map(|row| row.chars().collect())
             .collect::<Vec<Vec<char>>>())
}


pub fn run_challenge_04_00(input_name: &str) -> i32 {
    let input = read_by_line(&("./src/challenges/challenge_04/".to_owned() + input_name));

    let grid: Grid = build_array(&input);

    grid.sum_items_with_n_or_fewer_neighbours(4)
}

pub fn run_challenge_04_01(input_name: &str) -> i32 {
    let input = read_by_line(&("./src/challenges/challenge_04/".to_owned() + input_name));

    let mut grid: Grid = build_array(&input);

    let mut removables = grid.get_items_with_n_or_fewer_neighbours(4);

    let mut sum: i32 = 0;

    while removables.len() > 0 {
        for removable in &removables {
            grid.set(removable.x, removable.y, '.');
        }
        sum += removables.len() as i32;

        removables = grid.get_items_with_n_or_fewer_neighbours(4);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_01_sample() {
        assert_eq!(run_challenge_04_00("input_sample_a"), 13);
    }

    #[test]
    fn challenge_02_sample() {
        assert_eq!(run_challenge_04_01("input_sample_a"), 43);
    }
}

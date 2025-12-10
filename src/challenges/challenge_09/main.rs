use std::{cmp::Ordering, fmt, ops::Range};

use crate::util::read_by_line;

enum Direction {
    UP,
    UP_RIGHT,
    RIGHT,
    DOWN_RIGHT,
    DOWN,
    DOWN_LEFT,
    LEFT,
    UP_LEFT,
    CENTER,
}

#[derive(Debug, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn find_distance_to_bound(&self, bounds: &Rectangle) -> usize {
        let dist_up = bounds.end.y - self.y;
        let dist_down = self.y - bounds.start.y;
        let dist_right = bounds.end.x - self.x;
        let dist_left = self.x - bounds.start.x;

        dist_up.min(dist_down.min(dist_right.min(dist_left)))
    }

    /// The evidence of my errs
    pub fn find_nearest_bounding_edge(&self, bounds: &Rectangle) -> Direction {
        let dist_up = bounds.end.y - self.y;
        let dist_down = self.y - bounds.start.y;
        let dist_right = bounds.end.x - self.x;
        let dist_left = self.x - bounds.start.x;

        if dist_up < dist_down {
            if dist_left < dist_right {
                if dist_left < dist_up {
                    return Direction::LEFT
                }
                if dist_up < dist_right {
                    return Direction::UP
                }
                return Direction::UP_LEFT
            }
            if dist_right < dist_left {
                if dist_right < dist_up {
                    return Direction::RIGHT
                }
                if dist_up < dist_right {
                    return Direction::UP
                }
                return Direction::UP_RIGHT
            }
        }
        if dist_down < dist_up {
            if dist_left < dist_right {
                if dist_left < dist_down {
                    return Direction::LEFT
                }
                if dist_down < dist_left {
                    return Direction::DOWN
                }
                return Direction::DOWN_LEFT
            }
            if dist_right < dist_left {
                if dist_right < dist_down {
                    return Direction::RIGHT
                }
                if dist_down < dist_right {
                    return Direction::DOWN
                }
                return Direction::DOWN_RIGHT
            }
        }

        Direction::CENTER
    }
}

/// Parse '\n' seperated coordinate values 'x,y'
fn parse_coord_by_line(input: &str) -> Vec<Coordinate> {
    let mut coordinates: Vec<Coordinate> = Vec::new();

    for line in input.lines() {
        let split: Vec<usize> = line.split(",")
                                    .map(|x| x.trim().parse::<usize>().unwrap())
                                    .collect();
        coordinates.push(Coordinate { x: split[0], y: split[1] });
    }

    coordinates
}

fn find_coordinate_bounds(coords: &Vec<Coordinate>) -> Coordinate {
    let mut bounds = Coordinate { x: 0, y: 0 };

    for coord in coords {
        if coord.x > bounds.x { bounds.x = coord.x }
        if coord.y > bounds.y { bounds.y = coord.y }
    }

    bounds
}

#[derive(PartialEq, Clone)]
enum Tile {
    OTHER,
    RED,
    GREEN,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::OTHER => { write!(f, ".") },
            Tile::RED => { write!(f, "#") },
            Tile::GREEN => { write!(f, "X") },
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::OTHER => { write!(f, ".") },
            Tile::RED => { write!(f, "#") },
            Tile::GREEN => { write!(f, "X") },
        }
    }
}

/// Using an actual 2D array isn't ideal for this, but it's flexible going
///  into part two, and I just want to do it this way.
fn create_grid(coords: &Vec<Coordinate>) -> Vec<Vec<Tile>> {
    let mut grid: Vec<Vec<Tile>> = Vec::new();
    let bounds: Coordinate = find_coordinate_bounds(coords);

    for y in 0..bounds.y+3 {
        grid.push(Vec::new());

        for x in 0..bounds.x+3 {
            grid[y].push(Tile::OTHER);
        }
    }

    for coord in coords {
        grid[coord.y][coord.x] = Tile::RED;
    }

    grid
}

fn print_grid(grid: &Vec<Vec<Tile>>) {
    for row in grid {
        for item in row {
            print!("{}", item);
        }
        println!();
    }
}

#[derive(Debug)]
struct Rectangle {
    start: Coordinate,
    end: Coordinate,
}

impl Rectangle {
    pub fn new() -> Self {
        Rectangle { start: Coordinate::new(), end: Coordinate::new() }
    }

    pub fn from(a: &Coordinate, b: &Coordinate) -> Self {
        let start = Coordinate {
            x: a.x.min(b.x),
            y: a.y.min(b.y),
        };
        let end = Coordinate {
            x: a.x.max(b.x),
            y: a.y.max(b.y),
        };

        Rectangle { start, end }
    }

    pub fn area(&self) -> usize {
        let width = self.end.x - self.start.x +1;
        let height = self.end.y - self.start.y +1;
        return (width * height);
    }
}

fn find_largest_rect(coords: &Vec<Coordinate>, bounds: &Rectangle) -> Rectangle {
    let mut largest_rect = Rectangle::new();

    let mut sorted_coords = coords.clone();
    sorted_coords.sort_by(|a, b| {
        a.find_distance_to_bound(&bounds).cmp(&b.find_distance_to_bound(&bounds))
    });

    for coord_a in sorted_coords.iter() {
        for coord_b in sorted_coords[1..].iter() {
            let new_rect = Rectangle::from(coord_a, coord_b);
            if new_rect.area() > largest_rect.area() { largest_rect = new_rect; }
        }
    }

    largest_rect
}

fn find_bounds(coords: &Vec<Coordinate>) -> Rectangle {
    let mut bounds = Rectangle::new();

    for coord in coords {
        if coord.x > bounds.end.x { bounds.end.x = coord.x }
        if coord.y > bounds.end.y { bounds.end.y = coord.y }
    }

    bounds
}


pub fn run_challenge_09_00(input_name: &str) -> u64 {
    let input = read_by_line(&("./src/challenges/challenge_09/".to_owned() + input_name));

    let coordinates = parse_coord_by_line(&input);
    let bounds = find_bounds(&coordinates);

    let largest_rect = find_largest_rect(&coordinates, &bounds);
    println!("{:?}", largest_rect);

    largest_rect.area() as u64
}

/// Crushes all non-red tiles out of a grid and returns Vecs
///  of crushed rows and crushed columns
fn compress_grid(grid: &Vec<Vec<Tile>>) -> (
        Vec<Vec<Tile>>, Vec<usize>, Vec<usize>
    ) {
    let mut compressed_grid = grid.clone();

    let mut rows_to_squash: Vec<usize> = Vec::new();
    for (idx, row) in compressed_grid.iter().enumerate() {
        let mut squash: bool = true;
        for item in row {
            if *item == Tile::RED {
                squash = false;
                break;
            }
        }
        if squash { rows_to_squash.push(idx) }
    }

    let squashed_rows = rows_to_squash.clone();

    for row_idx in (0..compressed_grid.len()).rev() {
        if rows_to_squash.len() == 0 { break }

        if *rows_to_squash.last().unwrap() == row_idx {
            compressed_grid.remove(row_idx);
            rows_to_squash.pop();
        }
    }

    let mut columns_to_squash: Vec<usize> = Vec::new();
    for x in 0..compressed_grid[0].len() {
        let mut squash: bool = true;
        for y in 0..compressed_grid.len() {
            if compressed_grid[y][x] == Tile::RED {
                squash = false;
                break;
            }
        }
        if squash { columns_to_squash.push(x) }
    }

    let squashed_columns = columns_to_squash.clone();

    for col_idx in (0..compressed_grid[0].len()).rev() {
        if columns_to_squash.len() == 0 { break }

        if *columns_to_squash.last().unwrap() == col_idx {
            for row_idx in (0..compressed_grid.len()).rev() {
                compressed_grid[row_idx].remove(col_idx);
            }
            columns_to_squash.pop();
        }
    }

    (compressed_grid, squashed_rows, squashed_columns)
}

fn connect_coordinates(grid: &mut Vec<Vec<Tile>>, a: &Coordinate, b: &Coordinate) {
    if a.x == b.x {
        let mut range: Range<usize>;
        if a.y < b.y {
            range = a.y+1..b.y;
        } else {
            range = b.y+1..a.y;
        }
        for y in range {
            grid[y][a.x] = Tile::GREEN;
        }
    } else {
        // In this case, y will always be aligned if x is not
        let mut range: Range<usize>;
        if a.x < b.x {
            range = a.x+1..b.x;
        } else {
            range = b.x+1..a.x;
        }
        for x in range {
            grid[a.y][x] = Tile::GREEN;
        }
    }
}

/// Using an actual 2D array isn't ideal for this, but it's flexible going
///  into part two, and I just want to do it this way.
fn create_grid_two(coords: &Vec<Coordinate>) -> Vec<Vec<Tile>> {
    let mut grid: Vec<Vec<Tile>> = Vec::new();
    let bounds: Coordinate = find_coordinate_bounds(coords);

    let mut last_coordinate: Coordinate = coords.last().unwrap().clone();

    for y in 0..bounds.y+3 {
        grid.push(Vec::new());

        for x in 0..bounds.x+3 {
            grid[y].push(Tile::OTHER);
        }
    }

    for (idx, coord) in coords.iter().enumerate() {
        grid[coord.y][coord.x] = Tile::RED;

        connect_coordinates(&mut grid, &coord, &last_coordinate);
        last_coordinate = coord.clone();
    }

    grid
}

fn find_direction_clockwise(coord_a: &Coordinate, coord_b: &Coordinate, coord_c: &Coordinate) -> Direction {
    Direction::RIGHT
}

fn find_largest_rect_two(grid: &Vec<Vec<Tile>>, coords: &Vec<Coordinate>) -> usize {
    let largest_area: usize = 0;

    for (idx, coord) in coords.iter().enumerate() {
        let last_coord: Coordinate;
        if idx == 0 {
            last_coord = coords.last().unwrap().clone();
        } else {
            last_coord = coords[idx-1].clone();
        }
        let next_coord: Coordinate;
        if idx == coords.len()-1 {
            next_coord = coords.first().unwrap().clone();
        } else {
            next_coord = coords[idx+1].clone();
        }

        let direction: Direction = find_direction_clockwise(&last_coord, &coord, &next_coord);
    }

    largest_area
}

fn does_shape_have_three_points_straight_line(coords: &Vec<Coordinate>) -> bool {
    for (idx, coord) in coords.iter().enumerate() {
        let last_coord: Coordinate;
        if idx == 0 {
            last_coord = coords.last().unwrap().clone();
        } else {
            last_coord = coords[idx-1].clone();
        }
        let next_coord: Coordinate;
        if idx == coords.len()-1 {
            next_coord = coords.first().unwrap().clone();
        } else {
            next_coord = coords[idx+1].clone();
        }

        if coord.x == last_coord.x && coord.x == next_coord.x {
            return true
        }
        if coord.y == last_coord.y && coord.y == next_coord.y {
            return true
        }
    }

    false
}

pub fn run_challenge_09_01(input_name: &str) -> u64 {
    let input = read_by_line(&("./src/challenges/challenge_09/".to_owned() + input_name));


    let coordinates = parse_coord_by_line(&input);
    println!("EES EET STOOPUD?!?! {}", does_shape_have_three_points_straight_line(&coordinates));

    let bounds = find_bounds(&coordinates);
    let grid = create_grid_two(&coordinates);

    print_grid(&grid);


    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_01_sample() {
        assert_eq!(run_challenge_09_00("input_sample_a"), 50);
    }

    #[test]
    fn challenge_02_sample() {
        assert_eq!(run_challenge_09_01("input_sample_a"), 24);
    }
}

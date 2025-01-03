use std::{fmt, fs::{self, File}, io::Write};

pub fn read_in_file(input_filename: &str) -> Vec<String> {
    let error_msg = format!("unable to read file {}", input_filename);
    let input_data = fs::read_to_string(input_filename).expect(error_msg.as_str());
    let split = input_data.lines();
    let file_lines: Vec<String> = split.map(str::to_string).collect();
    return file_lines.clone();
}

pub fn append_to_file(output_filename: String, output_data: String) {
    let error_msg = format!("unable to write to file {}", output_filename);
    let mut f = File::options().append(true).open(output_filename).unwrap();
    writeln!(&mut f, "{}", output_data);
}

pub fn print_2d_map(input_map: &Vec<Vec<i32>>) {
    for row in input_map {
        for col in row {
            print!("{}", col);
        }
        print!("\n");
    }
}
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            '>' => Self::Right,
            '<' => Self::Left,
            _ => panic!("Bad char input for Direction::from"),
        }
    }
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        return [
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right,
        ].iter().copied();
    }

    pub fn reverse(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Clone, PartialEq, Hash, Eq, Ord, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(r: {}, c: {})", self.row, self.col)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.row == other.row {
            return Some(self.col.cmp(&other.col));
        } else {
            return Some(self.row.cmp(&other.row));
        }
    }
}

impl Position {
    pub fn walk(&self, steps: usize, direction: Direction) -> Self {
        let mut new_position = self.clone();
        match direction {
            Direction::Up => new_position.row -= steps,
            Direction::Down => new_position.row += steps,
            Direction::Left => new_position.col -= steps,
            Direction::Right => new_position.col += steps,
        }
        return new_position
    }

    pub fn direction(&self, next_position: Position) -> Direction {
        if self.row > next_position.row {
            return Direction::Up;
        } else if self.row < next_position.row {
            return Direction::Down;
        } else if self.col > next_position.col {
            return Direction::Left;
        } else if self.col < next_position.col {
            return Direction::Right;
        } else {
            panic!("Points are the same!");
        }
    }

    pub fn get_neighbours(&self, max_row: usize, max_col: usize) -> Vec<Position> {
        let mut row: i32 = self.row as i32;
        let mut col: i32 = self.col as i32;
        let mut neighbours: Vec<Position> = Vec::new();

        for d in Direction::iter() {
            match d {
                Direction::Up => {
                    if self.row > 0 {
                        neighbours.push(self.walk(1, d));
                    }
                },
                Direction::Down => {
                    if self.row < max_row - 1 {
                        neighbours.push(self.walk(1, d));
                    }
                },
                Direction::Left => {
                    if self.col > 0 {
                        neighbours.push(self.walk(1, d));
                    }
                },
                Direction::Right => {
                    if self.col < max_col - 1 {
                        neighbours.push(self.walk(1, d));
                    }
                },
            };
        }
        return neighbours;
    }

    pub fn manhattan_distance(&self, target: &Self) -> usize {
        return self.row.abs_diff(target.row) + self.col.abs_diff(target.col);
    }

    pub fn delta(&self, target: &Self) -> (i32, i32) {
        return (self.row as i32 - target.row as i32, self.col as i32 - target.col as i32);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        return Self{x: x as i32, y: y as i32};
    }

    pub fn walk(&self, x: i32, y: i32) -> Self {
        return Self{ x: self.x + x, y: self.y + y };
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Point64 {
    pub x: i64,
    pub y: i64,
}

impl Point64 {
    pub fn new(x: i64, y: i64) -> Self {
        return Self{x: x, y: y};
    }

    pub fn walk(&self, x: i64, y: i64) -> Self {
        return Self{ x: self.x + x, y: self.y + y };
    }

    pub fn walk32(&self, x: i32, y: i32) -> Self {
        return Self{ x: self.x + x as i64, y: self.y + y as i64 };
    }
}
use std::{fmt, fs};

pub fn read_in_file(input_filename: &str) -> Vec<String> {
    let error_msg = format!("unable to read file {}", input_filename);
    let input_data = fs::read_to_string(input_filename).expect(error_msg.as_str());
    let split = input_data.lines();
    let file_lines: Vec<String> = split.map(str::to_string).collect();
    return file_lines.clone();
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

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        return [
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right,
        ].iter().copied();
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
}
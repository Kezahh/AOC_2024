const INPUTS_FOLDER: &str = "inputs/day_4";

use crate::generic;


const XMAS_LENGTH: usize = 4;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    DiagonalUpLeft,
    DiagonalDownLeft,
    DiagonalUpRight,
    DiagonalDownRight
}

impl Direction {
    fn iter() -> impl Iterator<Item = Direction> {
        return [
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right,
            Self::DiagonalUpLeft,
            Self::DiagonalDownLeft,
            Self::DiagonalUpRight,
            Self::DiagonalDownRight
        ].iter().copied();
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Letter {
    X,
    M,
    A,
    S,
    None,
}

impl From<char> for Letter {
    fn from(value: char) -> Self {
        match value {
            'X' => Letter::X,
            'M' => Letter::M,
            'A' => Letter::A,
            'S' => Letter::S,
            _ => Letter::None,
        }
    }
}

impl Letter {
    fn to_char(&self) -> char {
        match self {
            Letter::X => 'X',
            Letter::M => 'M',
            Letter::A => 'A',
            Letter::S => 'S',
            Letter::None => '.',
        }
    }
}

struct LetterMap {
    rows: Vec<Vec<Letter>>,
}

impl From<Vec<String>> for LetterMap {
    fn from(value: Vec<String>) -> Self {
        return Self{rows: value.iter()
        .map(|x| x.chars().map(|y| Letter::from(y)).collect::<Vec<Letter>>()).collect::<Vec<Vec<Letter>>>()};
    }
}

impl From<Vec<&str>> for LetterMap {
    fn from(value: Vec<&str>) -> Self {
        return Self::from(value.iter().map(|x| x.to_string()).collect::<Vec<String>>());
    }
}

impl LetterMap {
    fn row_count(&self) -> usize {
        return self.rows.len();
    }

    fn col_count(&self) -> usize {
        return self.rows[0].len();
    }

    fn get_letter(&self, row: usize, col: usize) -> Letter {
        return self.rows[row][col].clone();
    }

    fn count_xmas(&self, row: usize, col: usize) -> usize {
        let mut temp_sum: usize = 0;
        for d in Direction::iter() {
            if self.check_direction(d, row, col) {
                temp_sum += 1;
            }
        }
        return temp_sum;
    }

    fn is_cross(&self, row: usize, col: usize) -> bool {
        let mut temp_sum: usize = 0;
        if self.check_cross_exceeds(row, col) {
            return false;
        } else {
            let diagonals: Vec<Letter> = self.get_diagonals_vec(row, col);
            let diagonals_string: String = diagonals.iter().map(|x| x.to_char()).collect::<String>();
            let is_cross_bool: bool =
                diagonals_string.as_str() == "MMSS" ||
                diagonals_string.as_str() == "SSMM" ||
                diagonals_string.as_str() == "MSSM" ||
                diagonals_string.as_str() == "SMMS";
            if is_cross_bool {
                println!("row: {}, col: {}, diagonals_string = {:?}, is_cross = {}", row, col, diagonals_string, is_cross_bool);
            }
            
            
            return is_cross_bool;
        }
    }

    fn get_diagonals_vec(&self, row: usize, col: usize) -> Vec<Letter> {
        // returns the diagonals in a loop CW from top-left, top-right, bottom-right, bottom-left
        return vec![
            self.get_letter(row - 1, col - 1),
            self.get_letter(row - 1, col + 1),
            self.get_letter(row + 1, col + 1),
            self.get_letter(row + 1, col - 1),
        ];
    }

    fn check_cross_exceeds(&self, row: usize, col: usize) -> bool {
        return 
            row < 1 || row >= self.row_count() - 1 || col < 1 || col >= self.col_count() - 1;
    }

    fn check_direction_exceeds(&self, direction: Direction, row: usize, col: usize) -> bool {
        match direction {
            Direction::Up => {
                if (row < XMAS_LENGTH - 1) {
                    return true;
                }
            }
            Direction::Down => {
                if (row + XMAS_LENGTH > self.row_count()) {
                    return true;
                }
            }
            Direction::Left => {
                if (col < XMAS_LENGTH - 1) {
                    return true;
                }
            }
            Direction::Right => {
                if (col + XMAS_LENGTH > self.col_count()) {
                    return true;
                }
            }
            Direction::DiagonalUpLeft => {
                return self.check_direction_exceeds(Direction::Up, row, col) || self.check_direction_exceeds(Direction::Left, row, col);
            }
            Direction::DiagonalDownLeft => {
                return self.check_direction_exceeds(Direction::Down, row, col) || self.check_direction_exceeds(Direction::Left, row, col);
            }
            Direction::DiagonalUpRight => {
                return self.check_direction_exceeds(Direction::Up, row, col) || self.check_direction_exceeds(Direction::Right, row, col);
            }
            Direction::DiagonalDownRight => {
                return self.check_direction_exceeds(Direction::Down, row, col) || self.check_direction_exceeds(Direction::Right, row, col);
            }
        }
        return false;
    }

    fn check_direction(&self, direction: Direction, row: usize, col: usize) -> bool {
        if self.check_direction_exceeds(direction, row, col) {
            return false;
        }

        match direction {
            Direction::Up => {
                return
                    self.get_letter(row - 1, col) == Letter::M &&
                    self.get_letter(row - 2, col) == Letter::A &&
                    self.get_letter(row - 3, col) == Letter::S;
            }
            Direction::Down => {
                return
                    self.get_letter(row + 1, col) == Letter::M &&
                    self.get_letter(row + 2, col) == Letter::A &&
                    self.get_letter(row + 3, col) == Letter::S;
            }
            Direction::Left => {
                return
                    self.get_letter(row, col - 1) == Letter::M &&
                    self.get_letter(row, col - 2) == Letter::A &&
                    self.get_letter(row, col - 3) == Letter::S;
            }
            Direction::Right => {
                return
                    self.get_letter(row, col + 1) == Letter::M &&
                    self.get_letter(row, col + 2) == Letter::A &&
                    self.get_letter(row, col + 3) == Letter::S;
            }
            Direction::DiagonalUpLeft => {
                return
                    self.get_letter(row - 1, col - 1) == Letter::M &&
                    self.get_letter(row - 2, col - 2) == Letter::A &&
                    self.get_letter(row - 3, col - 3) == Letter::S;
            }
            Direction::DiagonalDownLeft => {
                return
                    self.get_letter(row + 1, col - 1) == Letter::M &&
                    self.get_letter(row + 2, col - 2) == Letter::A &&
                    self.get_letter(row + 3, col - 3) == Letter::S;
            }
            Direction::DiagonalUpRight => {
                return
                    self.get_letter(row - 1, col + 1) == Letter::M &&
                    self.get_letter(row - 2, col + 2) == Letter::A &&
                    self.get_letter(row - 3, col + 3) == Letter::S;
            }
            Direction::DiagonalDownRight => {
                return
                    self.get_letter(row + 1, col + 1) == Letter::M &&
                    self.get_letter(row + 2, col + 2) == Letter::A &&
                    self.get_letter(row + 3, col + 3) == Letter::S;
            }
        }
    }
}



fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let letter_map: LetterMap = LetterMap::from(input_lines);

    let mut row: usize;
    let mut col: usize;
    let mut xmas_sum: usize = 0;

    for row in 0..letter_map.row_count() {
        for col in 0..letter_map.col_count() {
            if !part_2 {
                if letter_map.get_letter(row, col) == Letter::X {
                    xmas_sum += letter_map.count_xmas(row, col);
                }
            } else {
                if letter_map.get_letter(row, col) == Letter::A {
                    if letter_map.is_cross(row, col) {
                        xmas_sum += 1;
                    }
                }
            }
        }   
    }

    return xmas_sum;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here
        let letter_map: LetterMap = LetterMap::from(vec!["AAAAAA", "AXMASA", "AAAAAA", "AAAAAA", "AAAAAA", "AAAAAA"]);
        assert!(letter_map.check_direction_exceeds(Direction::Up, 0, 0) == true);
        assert!(letter_map.check_direction_exceeds(Direction::Up, 1, 0) == true);
        assert!(letter_map.check_direction_exceeds(Direction::Up, 2, 0) == true);
        assert!(letter_map.check_direction_exceeds(Direction::Right, 0, 5) == true);
        assert!(letter_map.check_direction_exceeds(Direction::Right, 0, 4) == true);
        assert!(letter_map.check_direction_exceeds(Direction::Right, 0, 3) == true);
        assert!(letter_map.check_direction_exceeds(Direction::Down, 5, 0) == true);
        assert!(letter_map.check_direction_exceeds(Direction::Down, 4, 0) == true);
        assert!(letter_map.check_direction_exceeds(Direction::Down, 3, 0) == true);
        assert!(letter_map.check_direction_exceeds(Direction::Left, 0, 0) == true);
        assert!(letter_map.check_direction_exceeds(Direction::Left, 0, 1) == true);
        assert!(letter_map.check_direction_exceeds(Direction::Left, 0, 2) == true);

        assert!(letter_map.check_direction_exceeds(Direction::DiagonalUpLeft, 0, 3) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalUpLeft, 1, 3) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalUpLeft, 2, 3) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalUpLeft, 3, 0) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalUpLeft, 3, 1) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalUpLeft, 3, 2) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalDownLeft, 5, 3) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalDownLeft, 4, 3) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalDownLeft, 3, 3) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalDownLeft, 2, 0) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalDownLeft, 2, 1) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalDownLeft, 2, 2) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalUpRight, 0, 0) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalUpRight, 1, 0) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalUpRight, 2, 0) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalUpRight, 3, 5) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalUpRight, 3, 4) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalUpRight, 3, 3) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalDownRight, 5, 0) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalDownRight, 4, 0) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalDownRight, 3, 0) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalDownRight, 2, 5) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalDownRight, 2, 4) == true);
        assert!(letter_map.check_direction_exceeds(Direction::DiagonalDownRight, 2, 3) == true);
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 18);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 2543);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_2.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 9);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 1930);
        // 1988 too high
    }
}
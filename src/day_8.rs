const INPUTS_FOLDER: &str = "inputs/day_8";

use std::collections::{HashMap, HashSet};

use crate::generic;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn new(row: i32, col: i32) -> Self {
        return Self{row: row, col: col};
    }
}


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let char_map: Vec<Vec<char>> = input_lines.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

    for r in 0..char_map.len() {
        for c in 0..char_map[0].len() {
            let current_char = char_map[r][c];
            if current_char != '.' {
                if !antennas.contains_key(&current_char) {
                    antennas.insert(current_char, Vec::new());
                }
                antennas.get_mut(&current_char).unwrap().push(Position::new(r as i32, c as i32));
            }
        }
    }

    let mut antinodes: HashSet<Position> = HashSet::new();

    for (k, v) in antennas.iter() {
        for i in 0..(v.len() - 1) {
            for j in (i + 1)..v.len() {
                let p1: &Position = &v[i];
                let p2: &Position = &v[j];
                
                let vertical_delta: i32 = p2.row - p1.row;
                let horizontal_delta: i32 = p2.col - p1.col;
                
                if !part_2 {
                    antinodes.insert(Position::new(p2.row + vertical_delta, p2.col + horizontal_delta));
                    antinodes.insert(Position::new(p1.row - vertical_delta, p1.col - horizontal_delta));
                } else {
                    let mut i: i32 = 1;

                    antinodes.insert(p1.clone());
                    antinodes.insert(p2.clone());

                    loop {
                        let new_point = Position::new(p2.row + (vertical_delta * i), p2.col + (horizontal_delta * i));
                        if new_point.row < 0 || new_point.row >= char_map.len() as i32 || new_point.col < 0 || new_point.col >= char_map[0].len()  as i32 {
                            break;
                        }
                        antinodes.insert(new_point);
                        i += 1;
                    }

                    i = 1;
                    loop {
                        let new_point = Position::new(p1.row - (vertical_delta * i), p1.col - (horizontal_delta * i));
                        if new_point.row < 0 || new_point.row >= char_map.len() as i32 || new_point.col < 0 || new_point.col >= char_map[0].len()  as i32 {
                            break;
                        }
                        antinodes.insert(new_point);
                        i += 1;
                    }
                }
            }
        }
    }

    for r in 0..char_map.len() {
        let mut row_string: String = String::new();
        for c in 0..char_map[0].len() {
            if antinodes.contains(&Position::new(r as i32, c as i32)) {
                row_string.push('#');
            } else {
                row_string.push('.');
            }
        }
        println!("{}", row_string);
    }


    return antinodes.iter()
        .filter(|x| x.row < char_map.len() as i32 && x.row >= 0 && x.col < char_map[0].len() as i32 && x.col >= 0)
        .collect::<Vec<&Position>>().len();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 14);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 376);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 34);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 1352);
    }
}
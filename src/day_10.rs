const INPUTS_FOLDER: &str = "inputs/day_10";

use std::{collections::HashSet, fmt};

use crate::generic;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn iter() -> impl Iterator<Item = Direction> {
        return [
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right,
        ].iter().copied();
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct Position {
    row: usize,
    col: usize,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(r: {}, c: {})", self.row, self.col)
    }
}

impl Position {
    fn walk(&self, steps: usize, direction: Direction) -> Self {
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

struct TrailMap {
    tiles: Vec<Vec<usize>>,
}

impl From<Vec<String>> for TrailMap {
    fn from(value: Vec<String>) -> Self {
        Self{tiles: value.iter().map(|x| x.chars().map(|y| y as usize - 48).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>()}
    }
}

impl TrailMap {
    fn get_row_count(&self) -> usize {
        return self.tiles.len();
    }

    fn get_col_count(&self) -> usize {
        return self.tiles[0].len();
    }

    fn get_pos(&self, row: usize, col: usize) -> usize {
        return self.tiles[row][col];
    }

    fn get_pos_from_pos(&self, p: &Position) -> usize {
        return self.tiles[p.row][p.col];
    }

    fn get_neighbours(&self, p: Position) -> Vec<Position> {
        let mut row: i32 = p.row as i32;
        let mut col: i32 = p.col as i32;
        let mut neighbours: Vec<Position> = Vec::new();

        for d in Direction::iter() {
            match d {
                Direction::Up => {
                    if p.row > 0 {
                        neighbours.push(p.walk(1, d));
                    }
                },
                Direction::Down => {
                    if p.row < self.get_row_count() - 1 {
                        neighbours.push(p.walk(1, d));
                    }
                },
                Direction::Left => {
                    if p.col > 0 {
                        neighbours.push(p.walk(1, d));
                    }
                },
                Direction::Right => {
                    if p.col < self.get_col_count() - 1 {
                        neighbours.push(p.walk(1, d));
                    }
                },
            };
        }
        return neighbours;
    }
}

fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut trail_map: TrailMap = TrailMap::from(input_lines);


    let mut starting_positions: Vec<Position> = Vec::new();
    for r in 0..trail_map.get_row_count() {
        for c in 0..trail_map.get_col_count() {
            if trail_map.get_pos(r, c) == 0 {
                starting_positions.push(Position { row: r, col: c });
            }
        }
    }

    let mut score_sum: usize = 0;
    let mut trail_heads_vec: Vec<Position>;
    let mut trail_heads: HashSet<&Position>;
    for s in starting_positions {
        trail_heads_vec = get_score(&s, &trail_map);
        if !part_2 {
            trail_heads = HashSet::from_iter(trail_heads_vec.iter());
            score_sum += trail_heads.len();
        } else {
            score_sum += trail_heads_vec.len();
        }
    }


    return score_sum;
}

fn get_score(p: &Position, trail_map: &TrailMap) -> Vec<Position> {
    let mut heads: Vec<Position> = Vec::new();
    let current_value = trail_map.get_pos_from_pos(p);
    // println!("{}Getting score for {:?}", "\t".to_string().repeat(current_value), p);
    if current_value == 9 {
        // println!("{}Found a 9", "\t".to_string().repeat(current_value));
        heads.push(p.clone());
        return heads;
    }
    let neighbours = trail_map.get_neighbours(p.clone());
    for n in neighbours {
        if trail_map.get_pos_from_pos(&n) == (current_value + 1) {
            heads.append(&mut get_score(&n, &trail_map));
        }
    }

    // println!("{}Returning score {}", "\t".to_string().repeat(current_value), heads.len());
    return heads;
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
        assert!(answer == 36);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 798);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 81);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 1816);
    }
}
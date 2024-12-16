const INPUTS_FOLDER: &str = "inputs/day_16";

use std::collections::{HashMap, HashSet};

use crate::generic::{self, Direction, Position};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Start,
    End
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '.' => Self::Empty,
            'S' => Self::Start,
            'E' => Self::End,
            _ => panic!("Bad char given to tile from"),
        }
    }
}

struct TileMap {
    tiles: Vec<Vec<Tile>>,
    start: Position,
    end: Position,
}

impl TileMap {
    fn new(value: Vec<String>) -> Self {
        let mut tiles = value.iter().map(|x| x.chars().map(|y| Tile::from(y)).collect::<Vec<Tile>>()).collect::<Vec<Vec<Tile>>>();
        let mut start: Position = Position { row: 0, col: 0 };
        let mut end: Position = Position { row: 0, col: 0 };
        let mut found_start: bool = false;
        let mut found_end: bool = false;
        for r in 0..tiles.len() {
            for c in 0..tiles[0].len() {
                if !found_start && tiles[r][c] == Tile::Start {
                    start = Position{ row: r, col: c };
                    tiles[r][c] = Tile::Empty;
                    found_start = true;
                } else if !found_end && tiles[r][c] == Tile::End {
                    end = Position{ row: r, col: c };
                    tiles[r][c] = Tile::Empty;
                    found_end = true;
                }
                if found_start && found_end {
                    break;
                }
            }
            if found_start && found_end {
                break;
            }
        }
        if !found_start || !found_end {
            panic!("Could not find start or end on map!");
        }
        return Self { tiles: tiles, start: start, end: end };
    }

    fn get_tile(&self, p: Position) -> Tile {
        return self.tiles[p.row][p.col];
    }

    fn get_path_cost(&self, position: Position, direction: Direction, tail: Option<HashSet<Position>>, visited: &mut HashMap<Position, Option<usize>>, depth: usize) -> Option<usize> {
        if visited.contains_key(&position) {
            // if visited.get(&position).unwrap().is_none() {
            //     println!("\tdict We're at position {:?}", position);
            //     return None;
            // }
        }
        
        let mut current_tail: HashSet<Position> = HashSet::new();
        if tail.is_some() {
            current_tail.extend(tail.unwrap());
        }

        if current_tail.contains(&position) {
            return None;
        } else {
            current_tail.insert(position);
        }
        // println!("{}Checking {:?} @ {:?}", " ".repeat(depth), position, direction);
        let mut neighbour_path_costs: Vec<usize> = Vec::new();
        for d in Direction::iter() {
            if d != direction.reverse() {
                let neighbour: Position = position.walk(1, d);
                if neighbour == self.end {
                    if d == direction {
                        visited.insert(position, Some(1));
                        return Some(1);
                    } else {
                        visited.insert(position, Some(1001));
                        return Some(1001);
                    }
                }
                if self.get_tile(neighbour) == Tile::Empty {
                    // println!("{}Checking neighbour{:?} @ {:?}", " ".repeat(depth), neighbour, direction);
                    let neighbour_cost: Option<usize> = self.get_path_cost(neighbour, d, Some(current_tail.clone()), visited, depth + 1);
                    if neighbour_cost.is_some() {
                        if d == direction {
                            neighbour_path_costs.push(1 + neighbour_cost.unwrap());
                        } else {
                            neighbour_path_costs.push(1 + 1000 + neighbour_cost.unwrap());
                        }
                    }
                }
            }
        }
        if neighbour_path_costs.len() == 0 {
            visited.insert(position, None);
            println!("We're at position {:?}", position);
            return None;
        }

        let min_cost: Option<usize> = Some(neighbour_path_costs.iter().copied().min().unwrap());
        visited.insert(position, min_cost);
        return min_cost;
    }

    fn get_neighbours(&self, position: Position) -> Vec<Position> {
        let mut neighbours: Vec<Position> = Vec::new();
        for d in Direction::iter() {
            match self.get_tile(position.walk(1, d)) {
                Tile::Empty => neighbours.push(position.walk(1, d)),
                _ => (),
            }
        }
        return neighbours;
    }

    fn col_count(&self) -> usize {
        return self.tiles[0].len();
    }

    fn row_count(&self) -> usize {
        return self.tiles.len();
    }

    fn print_map(&self) {
        for r in 0..self.row_count() {
            let mut row_string: String = String::new();
            for c in 0..self.col_count() {
                if r == self.start.row && c == self.start.col {
                    row_string.push('S');
                } else if r == self.end.row && c == self.end.col {
                    row_string.push('E');
                } else {
                    match self.get_tile(Position { row: r, col: c }) {
                        Tile::Wall => row_string.push('#'),
                        _ => row_string.push('.'),
                    }
                }
            }
            println!("{:?}", row_string);
        }
    }
}

fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut tile_map: TileMap = TileMap::new(input_lines);
    let mut visited: HashMap<Position, Option<usize>> = HashMap::new();

    tile_map.print_map();
    return tile_map.get_path_cost(tile_map.start, Direction::Right, None, &mut visited, 0).unwrap();
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
        assert!(answer == 7036);
    }

    #[test]
    fn example_1_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_2.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 11048);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 21138);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 30);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 7185540);
    }
}
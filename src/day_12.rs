const INPUTS_FOLDER: &str = "inputs/day_12";

use std::collections::{HashMap, HashSet};

use itertools::{sorted, Itertools};

use crate::generic;
use crate::generic::{Direction, Position};


#[derive(Debug)]
struct Region {
    character: char,
    positions: HashMap<usize, Vec<usize>>,
    positions_col: HashMap<usize, Vec<usize>>,
}

impl Region {
    fn new(character: char, positions_list: Vec<Position>) -> Self {
        let mut new_positions: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut new_positions_col: HashMap<usize, Vec<usize>> = HashMap::new();
        // println!("Creating region {} with positions {:?}", character, positions_list);
        for p in sorted(positions_list) {
            if !new_positions.contains_key(&p.row) {
                new_positions.insert(p.row, Vec::new());
            }
            if !new_positions_col.contains_key(&p.col) {
                new_positions_col.insert(p.col, Vec::new());
            }
            new_positions.get_mut(&p.row).unwrap().push(p.col);
            new_positions_col.get_mut(&p.col).unwrap().push(p.row);
        }

        return Self{character: character, positions: new_positions, positions_col: new_positions_col};
    }

    fn get_area(&self) -> usize {
        return self.positions.keys().map(|x| self.positions.get(x).unwrap().len()).sum::<usize>();
    }

    fn get_perimeter(&self, part_2: bool) -> usize {
        let mut vertical_bars: usize = 0;
        let mut vertical_bars_p2: usize = 0;
        let mut horizontal_bars: usize = 0;
        let mut horizontal_bars_p2: usize = 0;

        let mut last_col: usize = 0;
        let mut last_row_set: HashSet<(usize, Direction)> = HashSet::new();
        for row in sorted(self.positions.keys()) {
            let mut first_col: bool = true;
            let mut current_row_set: HashSet<(usize, Direction)> = HashSet::new();
            vertical_bars += 2;
            for col in self.positions.get(row).unwrap() {
                if first_col {
                    current_row_set.insert((*col, Direction::Down));
                }
                if *col > (last_col + 1) && !first_col {
                    vertical_bars += 2;
                    current_row_set.insert((last_col + 1, Direction::Up));
                    current_row_set.insert((*col, Direction::Down));
                }
                first_col = false;
                last_col = *col;
            }
            current_row_set.insert((last_col + 1, Direction::Up));
            vertical_bars_p2 += last_row_set.difference(&current_row_set).into_iter().collect::<Vec<&(usize, Direction)>>().len();
            println!("\tRow {}, vertical_bars_p2 = {}, current_row = {:?}", row, vertical_bars_p2, current_row_set);
            last_row_set = current_row_set;
        }
        vertical_bars_p2 += last_row_set.len();
        println!("\tRow end, vertical_bars_p2 = {}, last_row_set = {:?}", vertical_bars_p2, last_row_set);

        let mut last_row: usize = 0;
        let mut last_col_set: HashSet<(usize, Direction)> = HashSet::new();
        for col in sorted(self.positions_col.keys()) {
            let mut first_row: bool = true;
            let mut current_col_set: HashSet<(usize, Direction)> = HashSet::new();
            horizontal_bars += 2;
            for row in self.positions_col.get(col).unwrap() {
                if first_row {
                    current_col_set.insert((*row, Direction::Right));
                }
                if *row > (last_row + 1) && !first_row {
                    horizontal_bars += 2;
                    current_col_set.insert((last_row + 1, Direction::Left));
                    current_col_set.insert((*row, Direction::Right));
                }
                first_row = false;
                last_row = *row;
            }
            current_col_set.insert((last_row + 1, Direction::Left));
            horizontal_bars_p2 += last_col_set.difference(&current_col_set).into_iter().collect::<Vec<&(usize, Direction)>>().len();
            println!("\tCol {}, horizontal_bars_p2 = {}, current_col = {:?}", col, horizontal_bars_p2, current_col_set);
            last_col_set = current_col_set;
        }
        horizontal_bars_p2 += last_col_set.len();
        println!("\tCol end, horizontal_bars_p2 = {}, last_col_set = {:?}", horizontal_bars_p2, last_col_set);
        // println!("\tRegion {} has {} vertical and {} horizontal", self.character, vertical_bars, horizontal_bars);

        if !part_2 {
            return vertical_bars + horizontal_bars;
        } else {
            return vertical_bars_p2 + horizontal_bars_p2;
        }
    }
}

struct GardenMap {
    tiles: Vec<Vec<char>>,
}

impl From<Vec<String>> for GardenMap {
    fn from(value: Vec<String>) -> Self {
        let tiles: Vec<Vec<char>> = value.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        return Self{ tiles: tiles };
    }
}

impl GardenMap {
    fn get_row_count(&self) -> usize {
        return self.tiles.len();
    }

    fn get_col_count(&self) -> usize {
        return self.tiles[0].len();
    }

    fn get_pos(&self, row: usize, col: usize) -> char {
        return self.tiles[row][col];
    }

    fn get_pos_from_pos(&self, p: &Position) -> char {
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
    println!("Creating garden map");
    let garden_map: GardenMap = GardenMap::from(input_lines);
    
    println!("make used");
    let mut used: Vec<Vec<bool>> = vec![vec![false; garden_map.get_col_count()]; garden_map.get_row_count()];
    println!("make regions");
    let mut regions: Vec<Region> = Vec::new();

    for r in 0..garden_map.get_row_count() {
        for c in 0..garden_map.get_col_count() {
            if !used[r][c] {
                // println!("Doing row {}, col{}", r, c);
                regions.push(Region::new(garden_map.get_pos(r, c), get_region(Position{ row: r, col: c }, garden_map.tiles[r][c], &garden_map, &mut used)));
            }
        }
    }

    let mut sum_product: usize = 0;
    for r in regions.iter() {
        println!("region {} has area {}", r.character, r.get_area());
        println!("region {} has perimeter {}", r.character, r.get_perimeter(part_2));
        // println!("\t{:?}", r);
        sum_product += r.get_area() * r.get_perimeter(part_2);
        // break;
    }


    return sum_product;
}

fn get_region(p: Position, target_char: char, map: &GardenMap, used: &mut Vec<Vec<bool>>) -> Vec<Position> {
    let mut return_vec: Vec<Position> = Vec::new();
    let neighbours: Vec<Position> = map.get_neighbours(p);
    // println!("Doing point {:?}, neighbours = {:?}", p, neighbours);
    if neighbours.len() == 0 {
        return return_vec;
    }
    used[p.row][p.col] = true;
    return_vec.push(p);

    for n in neighbours.iter().filter(|x| map.get_pos_from_pos(x) == target_char) {
        // println!("\ttargeting neighbour {:?}", n);
        if !used[n.row][n.col] {
            return_vec.append(&mut get_region(n.clone(), target_char, map, used));
        }
    }

    return return_vec;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here
        let s1: HashSet<usize> = HashSet::from_iter([0, 2, 4, 6]);
        let s2: HashSet<usize> = HashSet::from_iter([0, 2, 5]);

        println!("DIfference len = {:?}", s2.difference(&s1).into_iter().collect::<Vec<&usize>>().len());
        
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 1930);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 1370100);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 1206);
    }

    #[test]
    fn example_2_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_2.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 80);
    }

    #[test]
    fn example_2_3() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_3.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 236);
    }

    #[test]
    fn example_2_4() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_4.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 368);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 818286);
        // 814074 too low
    }
}
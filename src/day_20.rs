const INPUTS_FOLDER: &str = "inputs/day_20";

use std::{collections::{HashMap, HashSet}};

use itertools::Itertools;

use crate::generic::{self, Position};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    Empty,
    Wall,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            _ => Self::Empty,
        }
    }
}

struct Map {
    start: Position,
    end: Position,
    tiles: Vec<Vec<Tile>>,
}

impl From<Vec<String>> for Map {
    fn from(value: Vec<String>) -> Self {
        let tiles: Vec<Vec<Tile>> = value.iter().map(|row| row.chars().map(|col| Tile::from(col)).collect::<Vec<Tile>>()).collect::<Vec<Vec<Tile>>>();
        let mut start: Option<Position> = None;
        let mut end: Option<Position> = None;
        for (row, string) in value.iter().enumerate() {
            for (col, char) in string.chars().enumerate() {
                match char {
                    'S' => start = Some(Position{row: row, col: col}),
                    'E' => end = Some(Position{row: row, col: col}),
                    _ => (),
                }
            }
        }

        return Map { start: start.unwrap(), end: end.unwrap(), tiles: tiles };
    }
}

impl Map {
    fn get_walls(&self) -> Vec<Position> {
        return self.tiles.iter().enumerate()
            .map(|(row, tiles)| {
                let mut row_tiles: Vec<Position> = Vec::new();
                for (col, tile) in tiles.iter().enumerate() {
                    if *tile == Tile::Wall {
                        row_tiles.push(Position{row: row, col: col});
                    }
                }
                return row_tiles;
            })
            .concat();
    }

    fn row_count(&self) -> usize {
        return self.tiles.len();
    }

    fn col_count(&self) -> usize {
        return self.tiles[0].len();
    }

    fn get_tile(&self, p: Position) -> Tile {
        return self.tiles[p.row][p.col].clone();
    }

    fn get_distance_to_start(&self, current_point: &Position, point_to_point: &HashMap<Position, Position>) -> Option<usize> {
        let mut point: Option<&Position> = Some(current_point);
        let mut distance: usize = 0;
        while point.is_some() && *point.unwrap() != self.start {
            point = point_to_point.get(point.unwrap());
            distance += 1;
        }

        if point.is_none() {
            return None
        } else {
            return Some(distance);
        }
    }

    fn get_path_to_start(&self, current_point: &Position, point_to_point: &HashMap<Position, Position>) -> Option<Vec<Position>> {
        let mut point: Option<&Position> = Some(current_point);
        let mut path: Vec<Position> = Vec::new();
        while point.is_some() && *point.unwrap() != self.start {
            path.push(*point.unwrap());
            point = point_to_point.get(point.unwrap());
        }

        if point.is_none() {
            return None
        } else {
            path.push(self.start);
            path.reverse();
            return Some(path);
        }
    }

    fn djikstra(&self, walls_vec: &Vec<Position>) -> Option<(usize, Vec<Position>)> {
        let walls: HashSet<Position> = HashSet::from_iter(walls_vec.clone());
        let mut point_to_point: HashMap<Position, Position> = HashMap::new();
        let mut points_to_do: Vec<Position> = vec![self.start];
        let mut done_points: HashSet<Position> = HashSet::new();
    
        while points_to_do.len() > 0 {
            // println!("distances = {:?}", point_distances_to_start);
            let current_point: Position = points_to_do.remove(0);
            // println!("Running djikstra on {:?}", current_point);
            if !done_points.contains(&current_point) {
                done_points.insert(current_point);
    
                let neighbours = current_point.get_neighbours(self.row_count(), self.col_count());
    
                // println!("current_point = {:?}, neighbours = {:?}", current_point, neighbours);
    
                for n in neighbours.iter().copied().filter(|x| !walls.contains(&x)) {
                    if current_point == self.start {
                        point_to_point.insert(current_point, current_point);
                        point_to_point.insert(n, current_point);
                        if !points_to_do.contains(&n) {
                            points_to_do.push(n);
                        }
                        done_points.remove(&n);
                    } else {
                        let distance_to_current: usize = self.get_distance_to_start(&current_point, &point_to_point).unwrap();
                        if point_to_point.contains_key(&n) {
                            let distance_to_neighbour: usize = self.get_distance_to_start(&n, &point_to_point).unwrap();
                            if distance_to_current + 1 < distance_to_neighbour {
                                point_to_point.insert(n, current_point);
                                if !points_to_do.contains(&n) {
                                    points_to_do.push(n);
                                }
                                done_points.remove(&n);
                            }
                        } else {
                            point_to_point.insert(n, current_point);
                            if !points_to_do.contains(&n) {
                                points_to_do.push(n);
                            }
                        }
                    }
                }
            }
        }

        let distance_to_start: Option<usize> = self.get_distance_to_start(&self.end, &point_to_point);
        let path_to_start: Option<Vec<Position>> = self.get_path_to_start(&self.end, &point_to_point);

        if distance_to_start.is_some() && path_to_start.is_some() {
            return Some((distance_to_start.unwrap(), path_to_start.unwrap()));
        } else {
            return None;
        }
    }

    fn get_cheat_ends(&self, current_point: &Position, path_points: &HashSet<Position>, depth: usize, cheat_ends: &mut HashMap<Position, usize>, tail: Option<HashSet<Position>>) {
        if depth == 20 {
            return;
        }

        let mut current_tail: HashSet<Position>;
        if tail.is_none() {
            current_tail = HashSet::new();
        } else {
            current_tail = tail.unwrap();
        }

        let neighbours: Vec<Position> = current_point.get_neighbours(self.row_count(), self.col_count());
        // println!("Getting cheat ends for {:?} with neighbours {:?}", current_point, neighbours);
        for n in neighbours {
            let neighbour_is_edge: bool = n.row ==0 || n.row == self.row_count() - 1 || n.col == 0 || n.col == self.col_count() - 1;
            if self.get_tile(n) == Tile::Empty && path_points.contains(&n) {
                if cheat_ends.contains_key(&n) {
                    let cheat_end_depth: usize = *cheat_ends.get(&n).unwrap();
                    if depth < cheat_end_depth {
                        // println!("\tadding {:?} at depth {}", n, depth);
                        cheat_ends.insert(n, depth);
                    }
                } else {
                    cheat_ends.insert(n, depth);
                }
            } else if self.get_tile(n) == Tile::Wall && !current_tail.contains(&n) {
                current_tail.insert(current_point.clone());
                self.get_cheat_ends(&n, path_points, depth + 1, cheat_ends, Some(current_tail.clone()));
            }
        }
    }
}


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let map = Map::from(input_lines);
    let walls: Vec<Position> = map.get_walls();

    let (base_time, base_path): (usize, Vec<Position>) = map.djikstra(&walls).unwrap();
    let base_path_hash: HashSet<Position> = HashSet::from_iter(base_path.clone());
    let mut cheat_times: HashMap<usize, usize> = HashMap::new();

    let max_cheat_distance: usize;
    if !part_2 {
        max_cheat_distance = 2;
    } else {
        max_cheat_distance = 20;
    }

    // println!("{:?}", base_path);
    for (i, p) in base_path.iter().enumerate() {
        let mut cheat_savings: HashMap<(Position, Position), usize> = HashMap::new();
        if i % 100 == 0 {
            println!("Running point {}/{}", i, base_path.len());
        }
        let mut cheat_ends: HashMap<Position, usize> = HashMap::new();
        for end_point in base_path[i..].iter() {
            let manhattan_distance = p.manhattan_distance(end_point);
            if manhattan_distance > 1 && manhattan_distance <= max_cheat_distance {
                cheat_ends.insert(*end_point, manhattan_distance);
            }
        }

        // println!("Cheat ends = {:?}", cheat_ends);

        let start_index: usize = base_path.iter().position(|x| *x == *p).unwrap();
        for cheat in cheat_ends.keys().sorted() {
            let end_index: usize = base_path.iter().position(|x| *x == *cheat).unwrap();
            let cheat_time_int: i32 = end_index as i32 - start_index as i32 - *cheat_ends.get(&cheat).unwrap() as i32;
            let cheat_time: usize;
            if cheat_time_int > 0 {
                cheat_time = cheat_time_int as usize;
                let cheat_path: (Position, Position);
                if start_index > end_index {
                    cheat_path = (*cheat, *p);
                } else {
                    cheat_path = (*p, *cheat);
                }
                // println!("cheat path {:?} saves {}", cheat_path, cheat_time);
                // println!("\tstart_index = {}", start_index);
                // println!("\tend_index = {}", end_index);
                // println!("\tactual time = {}", start_index.abs_diff(end_index));
                // println!("\tcheat cost = {}", cheat_ends.get(&cheat).unwrap());
                if cheat_savings.contains_key(&cheat_path) {
                    if cheat_time < *cheat_savings.get(&cheat_path).unwrap() {
                        cheat_savings.insert(cheat_path, cheat_time);
                    }
                } else {
                    cheat_savings.insert(cheat_path, cheat_time);
                }
            }
        }

        
        
        // if i == 5 {
        //     break;
        // }

        for cheat_time in cheat_savings.values() {
            if !cheat_times.contains_key(&cheat_time) {
                cheat_times.insert(*cheat_time, 1);
            } else {
                cheat_times.insert(*cheat_time, cheat_times.get(&cheat_time).unwrap() + 1);
            }
        }

        // println!("Cheat savings = {:?}", cheat_savings);
        // println!("Cheat times = {:?}", cheat_times);
    }


    let mut total_cheats: usize = 0;
    let target_difference: usize = 100;
    for time in cheat_times.keys().sorted() {
        let count_cheats: usize = *cheat_times.get(time).unwrap();
        println!("There are {} cheats that save {} picoseconds", count_cheats, time);
        if *time >= target_difference {
            total_cheats += count_cheats;
            // println!("Adding {} cheats from saving {} picoseconds, total = {}", count_cheats, time, total_cheats);
        }
    }

    return total_cheats;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here
        // let start_point
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 13);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 1409);

        // 393 too low
        // 5581 too high
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 285);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 1012821);
        // 1102213 too high
        // 1012821
        // 1243052
    }

}
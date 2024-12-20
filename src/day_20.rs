const INPUTS_FOLDER: &str = "inputs/day_20";

use std::collections::{HashMap, HashSet};

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

    fn djikstra(&self, walls_vec: &Vec<Position>) -> Option<usize> {
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
    
        return self.get_distance_to_start(&self.end, &point_to_point);
    }
}


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let map = Map::from(input_lines);
    let walls: Vec<Position> = map.get_walls();

    let base_time: usize = map.djikstra(&walls).unwrap();
    let mut cheat_times: HashMap<usize, usize> = HashMap::new();

    let mut walls_to_cheat: Vec<Position> = Vec::new();

    for w in walls.iter() {
        let neighbours: Vec<Position> = w.get_neighbours(map.row_count(), map.col_count());
        if neighbours.iter().filter(|x| map.get_tile(**x) == Tile::Empty).collect::<Vec<&Position>>().len() > 1 {
            walls_to_cheat.push(w.clone());
        }
    }

    for wall_index in 0..walls_to_cheat.len() {
        println!("Running wall {:4}/{}", wall_index, walls_to_cheat.len());
        let mut new_walls: Vec<Position> = walls.clone();
        new_walls.remove(new_walls.iter().position(|x| *x == walls_to_cheat[wall_index]).unwrap());

        let cheat_time: Option<usize> = map.djikstra(&new_walls);
        if cheat_time.is_some() && cheat_time.unwrap() < base_time {
            let cheat_difference: usize = base_time - cheat_time.unwrap();
            if !cheat_times.contains_key(&cheat_difference) {
                cheat_times.insert(cheat_difference, 1);
            } else {
                cheat_times.insert(cheat_difference, cheat_times.get(&cheat_difference).unwrap() + 1);
            }
        }
    }

    let mut total_cheats: usize = 0;
    let target_difference: usize = 100;
    for time in cheat_times.keys().sorted() {
        println!("There are {} cheats that save {} picoseconds", cheat_times.get(time).unwrap(), time);
        if *time <= target_difference {
            total_cheats += cheat_times.get(time).unwrap();
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
        assert!(answer == 5581);

        // 5581 too high
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
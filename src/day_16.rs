const INPUTS_FOLDER: &str = "inputs/day_16";

use std::{collections::{HashMap, HashSet}};

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

    fn djikstra(&self, position: Position, direction: Direction, done_set: &mut HashSet<Position>, distances: &mut HashMap<Position, Position>, neighbours_to_do: &mut Vec<(Position, Direction)>) {
        // println!("Running djikstra on {:?}", position);
        if done_set.contains(&position) {
            return;
        } else {
            done_set.insert(position);
        }
        let current_distance_to_end: usize = self.get_distance_recursive(position, &mut Vec::new(), distances);
        // println!("current distance to end = {}", current_distance_to_end);

        for (neighbour, neighbour_direction) in self.get_neighbours(position) {
            // println!("\tCheck neighbour {:?}, d = {:?}, direction = {:?}", neighbour, 0, neighbour_direction);
            if !distances.contains_key(&neighbour) {
                // println!("\t\tSet neighbour {:?} with distance {}", neighbour, 0 + current_distance_to_end);
                distances.insert(neighbour, position);
            } else {
                let neighbour_distance: usize;
                if direction.reverse() == neighbour_direction {
                    neighbour_distance = 1;
                } else {
                    neighbour_distance = 1001;
                }
                let neighbour_distance_to_end: usize = self.get_distance_recursive(neighbour, &mut Vec::new(), distances);
                // println!("\t\tAlready contains neighbour at distance {}", neighbour_distance_to_end);
                if neighbour_distance_to_end > (current_distance_to_end + neighbour_distance) {
                    // println!("\t\t\tReplacing neighbour distance to end. with {}", current_distance_to_end + neighbour_distance);
                    distances.insert(neighbour, position);
                } else {
                    // println!("\t\t\tKeeping neighbour distance to end. ignoring {}", current_distance_to_end + neighbour_distance);
                }
            }

            if !done_set.contains(&neighbour) {
                neighbours_to_do.push((neighbour, neighbour_direction));
            }
        }
    }

    fn get_distance_recursive(&self, position: Position, direction_list: &mut Vec<Direction>, distances: &mut HashMap<Position, Position>) -> usize {
        if position == self.end {
            return 0;
        }
        let next_point: Position = *distances.get(&position).unwrap();
        let next_direction: Direction = position.direction(next_point);
        direction_list.push(next_direction);
        // println!("\tFrom {:?} to {:?}", position, next_point);
        if next_point == self.end {
            let mut last_direction: Direction = direction_list.remove(0);
            let mut distance = 1;
            for d in direction_list {
                if *d == last_direction {
                    distance += 1;
                } else {
                    distance += 1001;
                    last_direction = *d;
                }
            }
            return distance
        }
        return self.get_distance_recursive(next_point, direction_list, distances);
    }

    fn get_ditance_to_end(&self, position: Position, distances: &mut HashMap<Position, Position>, debug: bool) -> usize {
        if position == self.start {
            return 0;
        }
        if debug {
            println!("Checking distance to end for {:?}", position);
            // println!("distances = {:?}", distances);
        }
        let mut last_point: Position = position.clone();
        let mut next_point: Position = *distances.get(&last_point).unwrap();
        let mut last_direction: Direction = last_point.direction(next_point);
        let mut total_distance: usize = 1;
        if next_point == self.start {

        }
        while next_point != self.start {
            if debug {
                println!("\tnext = {:?}, total_distance = {}", next_point, total_distance);
            }
            let direction = next_point.direction(last_point);
            if direction == last_direction {
                total_distance += 1;
            } else {
                total_distance += 1001;
            }
            last_direction = direction;
            last_point = next_point;
            next_point = *distances.get(&next_point).unwrap();
        }

        return total_distance;
    }

    fn get_neighbours(&self, position: Position) -> Vec<(Position, Direction)> {
        let mut neighbours: Vec<(Position, Direction)> = Vec::new();
        for d in Direction::iter() {
            let neighbour = position.walk(1, d);
            match self.get_tile(neighbour) {
                Tile::Empty => {
                    neighbours.push((position.walk(1, d), d));
                },
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

    fn get_empty_points(&self) -> HashSet<Position> {
        let mut empty_tiles: HashSet<Position> = HashSet::new();
        for r in 0..self.row_count() {
            for c in 0..self.col_count() {
                let p = Position { row: r, col: c };
                if self.get_tile(p) == Tile::Empty {
                    empty_tiles.insert(p);
                }
            }
        }
        return empty_tiles;
    }
}

fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut tile_map: TileMap = TileMap::new(input_lines);
    let mut distances: HashMap<Position, Position> = HashMap::new();
    let mut done_set: HashSet<Position> = HashSet::new();
    let mut neighbours_to_do: Vec<(Position, Direction)> = vec![(tile_map.end, Direction::Right)];

    tile_map.print_map();

    let mut i = 0;
    while neighbours_to_do.len() > 0 {
        let (next_neighbour, next_neighbour_direction): (Position, Direction) = neighbours_to_do.remove(0);
        tile_map.djikstra(next_neighbour, next_neighbour_direction, &mut done_set, &mut distances, &mut neighbours_to_do);
        i += 1;
        if i == 20 {
            // break;
        }
    }

    for i in 0..100 {
        done_set = HashSet::new();
        neighbours_to_do = vec![(tile_map.start, Direction::Right)];
        while neighbours_to_do.len() > 0 {
            let (next_neighbour, next_neighbour_direction): (Position, Direction) = neighbours_to_do.remove(0);
            tile_map.djikstra(next_neighbour, next_neighbour_direction, &mut done_set, &mut distances, &mut neighbours_to_do);
        }
    }

    // println!("getting targets");
    // let mut target: Position = Position { row: 10, col: 13 };
    // println!("Distance from {:?} is {}", target, tile_map.get_distance_recursive(target, &mut Vec::new(), &mut distances));
    // target = Position { row: 7, col: 10 };
    // println!("Distance from {:?} is {}", target, tile_map.get_distance_recursive(target, &mut Vec::new(), &mut distances));
    // target = Position { row: 7, col: 11 };
    // println!("Distance from {:?} is {}", target, tile_map.get_distance_recursive(target, &mut Vec::new(), &mut distances));
    // target = Position { row: 8, col: 11 };
    // println!("Distance from {:?} is {}", target, tile_map.get_distance_recursive(target, &mut Vec::new(), &mut distances));
    // target = Position { row: 9, col: 11 };
    // println!("Distance from {:?} is {}", target, tile_map.get_distance_recursive(target, &mut Vec::new(), &mut distances));
    // target = Position { row: 10, col: 11 };
    // println!("Distance from {:?} is {}", target, tile_map.get_distance_recursive(target, &mut Vec::new(), &mut distances));
    // target = Position { row: 7, col: 10 };
    // println!("Distance from {:?} is {}", target, tile_map.get_distance_recursive(target, &mut Vec::new(), &mut distances));
    // target = Position {row: 6, col: 9};
    // println!("Distance from {:?} is {}", target, tile_map.get_distance_recursive(target, &mut Vec::new(), &mut distances));

    println!("Getting final distance");
    return tile_map.get_distance_recursive(tile_map.start, &mut vec![Direction::Right], &mut distances) - 1;
    //10028
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
        assert!(answer == 143580);

        // 153536 too high
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
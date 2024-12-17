const INPUTS_FOLDER: &str = "inputs/day_16";

use std::{collections::{HashMap, HashSet}};

use itertools::all;

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

#[derive(Debug, Clone)]
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

    fn set_tile(&mut self, p: Position, value: Tile) {
        self.tiles[p.row][p.col] = value;
    }

    fn get_path_cost(&self, position: Position, direction: Direction, tail: Option<HashSet<Position>>, visited: &mut HashMap<Position, Option<usize>>, dead_ends: &mut HashSet<Position>, depth: usize) -> Option<usize> {
        if dead_ends.contains(&position) {
            return None;
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
        let mut neighbours: Vec<Position> = Vec::new();
        for d in Direction::iter() {
            if d != direction.reverse() {
                let neighbour: Position = position.walk(1, d);
                if neighbour == self.end {
                    println!("Found the end!");
                    if d == direction {
                        visited.insert(position, Some(1));
                        return Some(1);
                    } else {
                        visited.insert(position, Some(1001));
                        return Some(1001);
                    }
                }
                if self.get_tile(neighbour) == Tile::Empty {
                    neighbours.push(neighbour);
                    // println!("{}Checking neighbour{:?} @ {:?}", " ".repeat(depth), neighbour, direction);
                    let neighbour_cost: Option<usize> = self.get_path_cost(neighbour, d, Some(current_tail.clone()), visited, dead_ends, depth + 1);
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
            let mut all_dead_ends: bool = true;
            for n in neighbours {
                all_dead_ends = all_dead_ends && dead_ends.contains(&n);
            }
            if all_dead_ends {
                println!("Adding {:?} as dead end", position);
                dead_ends.insert(position);
            }
            visited.insert(position, None);
            // println!("We're at position {:?}", position);
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
        let current_distance_to_end_option: Option<usize> = self.get_distance_recursive(position, &mut Vec::new(), distances);
        if current_distance_to_end_option.is_none() {
            return;
        }
        let current_distance_to_end: usize = current_distance_to_end_option.unwrap();
        // if current_distance_to_end > 150000 {
        //     return;
        // }
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
                let neighbour_distance_to_end_option: Option<usize> = self.get_distance_recursive(neighbour, &mut Vec::new(), distances);
                
                // println!("\t\tAlready contains neighbour at distance {}", neighbour_distance_to_end);
                if neighbour_distance_to_end_option.is_none() || neighbour_distance_to_end_option.unwrap() > (current_distance_to_end + neighbour_distance) {
                    // println!("\t\t\tReplacing neighbour distance to end. with {}", current_distance_to_end + neighbour_distance);
                    distances.insert(neighbour, position);
                    neighbours_to_do.push((neighbour, neighbour_direction));
                    if done_set.contains(&neighbour) {
                        done_set.remove(&neighbour);
                    }
                } else {
                    // println!("\t\t\tKeeping neighbour distance to end. ignoring {}", current_distance_to_end + neighbour_distance);
                }
            }

            if !done_set.contains(&neighbour) {
                neighbours_to_do.push((neighbour, neighbour_direction));
            }
        }
    }

    fn get_distance_recursive(&self, position: Position, direction_list: &mut Vec<Direction>, distances: &mut HashMap<Position, Position>) -> Option<usize> {
        if position == self.end {
            return Some(0);
        }
        if !distances.contains_key(&position) {
            return None
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
            return Some(distance);
        }
        return self.get_distance_recursive(next_point, direction_list, distances);
    }

    fn get_path(&self, position: Position, distances: &HashMap<Position, Position>) -> Vec<Position> {
        if position == self.end {
            return vec![self.end];
        }
        let mut positions: Vec<Position> = vec![position];
        let next_point: Position = *distances.get(&position).unwrap();
        positions.append(&mut self.get_path(next_point, distances));
        return positions;
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
    // tile_map.set_tile(Position { row: 138, col: 7 }, Tile::Wall);

    while neighbours_to_do.len() > 0 {
        let (next_neighbour, next_neighbour_direction): (Position, Direction) = neighbours_to_do.remove(0);
        tile_map.djikstra(next_neighbour, next_neighbour_direction, &mut done_set, &mut distances, &mut neighbours_to_do);
    }

    for i in 0..2 {
        done_set = HashSet::new();
        neighbours_to_do = vec![(tile_map.start, Direction::Right)];
        while neighbours_to_do.len() > 0 {
            let (next_neighbour, next_neighbour_direction): (Position, Direction) = neighbours_to_do.remove(0);
            tile_map.djikstra(next_neighbour, next_neighbour_direction, &mut done_set, &mut distances, &mut neighbours_to_do);
        }
    }

    let best_path_length: usize = tile_map.get_distance_recursive(tile_map.start, &mut vec![Direction::Right], &mut distances).unwrap() - 1;

    // with blockages
    let all_path_points: Vec<Position> = tile_map.get_path(tile_map.start, &distances);
    let all_path_points_hash: HashSet<Position> = HashSet::from_iter(all_path_points.clone());
    let intersections: Vec<&Position> = all_path_points.iter().filter(|x| tile_map.get_neighbours(**x).len() > 2).collect::<Vec<&Position>>();

    let mut unique_points: HashSet<Position> = HashSet::from_iter(all_path_points.clone());

    let mut j: usize = 0;
    for (k, intersection) in intersections.iter().enumerate() {
        let intersection_index: usize = all_path_points.iter().position(|x| x == *intersection).unwrap();
        let neighbours: Vec<(Position, Direction)> = tile_map.get_neighbours(**intersection);
        // let mut block_sites: Vec<Position> = neighbours.clone();
        let mut block_sites: Vec<Position> = vec![all_path_points[intersection_index + 1]];
        // println!("Block sites for {:?} are {:?}", intersection, block_sites);
        let mut new_tile_map: TileMap = tile_map.clone();
        let mut block_index: usize = 0;
        while block_index < block_sites.len() {
            let mut b: Position = block_sites[block_index];
            new_tile_map.set_tile(b, Tile::Wall);

            // tile_map.print_map();
            distances = HashMap::new();
            done_set = HashSet::new();

            neighbours_to_do = vec![(new_tile_map.end, Direction::Right)];
            while neighbours_to_do.len() > 0 {
                let (next_neighbour, next_neighbour_direction): (Position, Direction) = neighbours_to_do.remove(0);
                new_tile_map.djikstra(next_neighbour, next_neighbour_direction, &mut done_set, &mut distances, &mut neighbours_to_do);
            }

            for i in 0..20 {
                done_set = HashSet::new();
                neighbours_to_do = vec![(new_tile_map.start, Direction::Right)];
                while neighbours_to_do.len() > 0 {
                    let (next_neighbour, next_neighbour_direction): (Position, Direction) = neighbours_to_do.remove(0);
                    new_tile_map.djikstra(next_neighbour, next_neighbour_direction, &mut done_set, &mut distances, &mut neighbours_to_do);
                }
            }

            let new_distance: Option<usize> = new_tile_map.get_distance_recursive(new_tile_map.start, &mut vec![Direction::Right], &mut distances);
            if new_distance.is_some() {
                println!("({}) New distance with blocker {:?}", k, b);
                println!("\t{:?}", new_distance.unwrap() - 1);
                if new_distance.unwrap() - 1 == best_path_length {
                    let full_path: Vec<Position> = new_tile_map.get_path(new_tile_map.start, &distances);
                    unique_points.extend(HashSet::<Position>::from_iter(full_path));
                    println!("\tunique points len = {:?}", unique_points.len());
                    let current_all_path_points: Vec<Position> = new_tile_map.get_path(tile_map.start, &distances);
                    let current_path_points_hash: HashSet<Position> = HashSet::from_iter(current_all_path_points.clone());
                    println!("\tThere are {} different intersections", current_path_points_hash.difference(&all_path_points_hash).collect::<Vec<&Position>>().len());

                    if neighbours.len() == 4 {
                        println!("\t{:?} has 4 neighbours", b);
                        let current_intersection_index_option: Option<usize> = current_all_path_points.iter().position(|x| x == *intersection);
                        if current_intersection_index_option.is_none() {
                            println!("\t\tCurrent intersection not included in new path.");
                        } else {
                            let current_intersection_index: usize = current_all_path_points.iter().position(|x| x == *intersection).unwrap();
                            block_sites.push(current_all_path_points[current_intersection_index + 1]);
                            println!("\t\tAdding new block site at {:?}", block_sites.last().unwrap());
                        }
                    }
                }
            }
            block_index += 1;
        }

        j += 1;
        if j == 10 {
            // break;
        }
    }


    if !part_2 {
        return best_path_length;
    } else {
        return unique_points.len();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here

        let mut mylist: Vec<usize> = vec![1,2,3];

        let mut i: usize = 0;
        while i < mylist.len() {
            let x = mylist[i];
            println!("{}", x);
            if x == 3 {
                mylist.push(44);
            }
            i += 1;
        }
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
        assert!(answer == 45);
    }

    #[test]
    fn example_2_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_2.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 64);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 645);
        //581 too low
        //600 too low
        // 615 is wrong
        // 620 is wrong
        // 622 is wrong (curiously correct for someone else....)
        //  627 maybe?
        // 631 is wrong (curiously correct for someone else....)
        // 638 is wrong
        // 641 is wrong
        // 644 is wrong
        //   645
        // 646 is wrong
        // 647 is wrong
        // 648 is wrong
        // 649 is wrong
        //650 too high
    }
}
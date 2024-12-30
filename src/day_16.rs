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
            } else if direction == last_direction.reverse() {
                panic!("Going backwards");
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

fn get_shortest_paths(points: &HashSet<Position>, max_row: usize, max_col: usize, start_position: &Position, start_direction: &Direction, end_position: &Position, my_score: usize, tail: &HashSet<&Position>, found_shortest: &mut Option<usize>, memoized: &mut HashMap<(Position, Direction, usize), Option<Vec<(usize, Vec<Position>)>>>, steps: usize) -> Option<Vec<(usize, Vec<Position>)>> {
    // println!("{:7} Doing {:?} with score {}", steps, start_position, my_score);
    if memoized.contains_key(&(*start_position, *start_direction, my_score)) {
        return memoized.get(&(*start_position, *start_direction, my_score)).unwrap().clone();
    }
    
    if found_shortest.is_some() {
        if my_score > found_shortest.unwrap() {
            return None;
        }
    }

    if start_position == end_position {
        if found_shortest.is_none() {
            *found_shortest = Some(my_score);
        } else if my_score < found_shortest.unwrap() {
            *found_shortest = Some(my_score);
        }
        let return_option: Option<Vec<(usize, Vec<Position>)>> = Some(vec![(my_score, vec![end_position.clone()])]);
        // memoized.insert((*start_position, *start_direction), return_option.clone());
        return return_option;
    }

    let mut my_tail: HashSet<&Position> = tail.clone();
    my_tail.insert(start_position);


    let neighbours: Vec<Position> = start_position.get_neighbours(max_row, max_col).into_iter().filter(|x| points.contains(&x) && !my_tail.contains(x)).collect::<Vec<Position>>();
    let mut shortest_paths: Vec<(usize, Vec<Position>)> = Vec::new();
    for n in neighbours {
        let neighbour_direction = start_position.direction(n);
        let mut neighbour_score: usize = my_score.clone();
        if neighbour_direction == *start_direction {
            neighbour_score += 1;
        } else {
            neighbour_score += 1001
        }
        let neighbour_path_option: Option<Vec<(usize, Vec<Position>)>> = get_shortest_paths(points, max_row, max_col, &n, &neighbour_direction, end_position, neighbour_score, &my_tail, found_shortest, memoized, steps + 1);
        if neighbour_path_option.is_some() {
            for path in neighbour_path_option.unwrap() {
                let mut path_from_point: Vec<Position> = vec![start_position.clone()];
                path_from_point.append(&mut path.1.clone());
                shortest_paths.push((path.0, path_from_point));
            }
        }        
    }

    
    if shortest_paths.len() > 0 {
        let shortest_path: usize = shortest_paths.iter().map(|x| x.0).min().unwrap();
        let return_option: Option<Vec<(usize, Vec<Position>)>> = Some(shortest_paths.iter().cloned().filter(|x| x.0 <= shortest_path).collect::<Vec<(usize, Vec<Position>)>>());
        memoized.insert((*start_position, *start_direction, shortest_path), return_option.clone());
        return return_option;
    } else {
        memoized.insert((*start_position, *start_direction, my_score), None);
        return None;
    }
}

fn get_path_score(path: &Vec<Position>, start_direction: &Direction) -> usize {
    let mut current_direction: Direction = start_direction.clone();
    let mut current_position: &Position = &path[0];
    let mut score: usize = 0;
    for p in path[1..].iter() {
        let next_direction = current_position.direction(*p);
        if next_direction == current_direction {
            score += 1;
        } else {
            score += 1001;
        }
        current_position = p;
        current_direction = next_direction;
    }

    return score;
}

fn djikstra_lowest_score(points: &HashSet<Position>, start_position: &Position, start_direction: &Direction) -> HashMap<(Position, Direction), usize> {
    let mut points_to_do: Vec<(Position, Direction)> = vec![(start_position.clone(), start_direction.clone())];
    let mut scores: HashMap<(Position, Direction), usize> = HashMap::new();
    let max_row: usize = points.iter().map(|x| x.row).max().unwrap() + 1;
    let max_col: usize = points.iter().map(|x| x.col).max().unwrap() + 1;
    scores.insert((*start_position, *start_direction), 0);

    while points_to_do.len() > 0 {
        let (target_point, target_direction): (Position, Direction) = points_to_do.remove(0);
        let target_point_score: usize = *scores.get(&(target_point, target_direction)).unwrap();
        let neighbours: Vec<Position> = target_point.get_neighbours(max_row, max_col).into_iter().filter(|x| points.contains(&x)).collect::<Vec<Position>>();
        // println!("Running {:?} with neighbours {:?}", target_point, neighbours);
        for n in neighbours {
            let neighbour_direction: Direction = target_point.direction(n);
            let mut neighbour_score: usize = target_point_score;
            if neighbour_direction == target_direction {
                neighbour_score += 1;
            } else {
                neighbour_score += 1001;
            }

            if scores.contains_key(&(n, neighbour_direction)) {
                let current_neighbour_score = scores.get_mut(&(n, neighbour_direction)).unwrap();
                if neighbour_score < *current_neighbour_score {
                    *current_neighbour_score = neighbour_score;
                    points_to_do.push((n, neighbour_direction));
                }
            } else {
                scores.insert((n, neighbour_direction), neighbour_score);
                points_to_do.push((n, neighbour_direction));
            }
        }
    }

    return scores;
}

fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let tile_map: TileMap = TileMap::new(input_lines);
    
    let scores_from_start: HashMap<(Position, Direction), usize> = djikstra_lowest_score(&tile_map.get_empty_points(), &tile_map.start, &Direction::Right);
    let shortest_score: usize = Direction::iter().map(|d| scores_from_start.get(&(tile_map.end, d))).filter(|x| x.is_some()).map(|x| *x.unwrap()).min().unwrap();
    let scores_from_end: HashMap<(Position, Direction), usize> = djikstra_lowest_score(&tile_map.get_empty_points(), &tile_map.end, &Direction::Down);

    let mut good_points_count: usize = 0;
    for p in tile_map.get_empty_points() {
        let mut finished_point = false;
        for d1 in Direction::iter() {
            let score_from_start: Option<&usize> = scores_from_start.get(&(p, d1));
            if score_from_start.is_some() {
                for d2 in Direction::iter() {
                    let score_from_end: Option<&usize> = scores_from_end.get(&(p, d2));
                    if score_from_end.is_some() {
                        let mut score_sum: usize = *score_from_start.unwrap() + *score_from_end.unwrap();
                        if d1 != d2.reverse() {
                            // when d1 == d2.reverse(), it's going in the same direction.
                            // otherwise add offset of extra 1000 for new turn.
                            score_sum += 1000;
                        }
                        if score_sum == shortest_score {
                            good_points_count += 1;
                            finished_point = true;
                            break;
                        }
                    }
                }
            }
            if finished_point {
                break;
            }
        }
    }
    
    if !part_2 {
        return shortest_score;
    } else {
        return good_points_count;
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
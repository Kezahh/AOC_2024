const INPUTS_FOLDER: &str = "inputs/day_21";

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::generic::{self, Direction, Position};

struct NumberPad {
    buttons: HashMap<char, Position>,
    points_map: HashMap<Position, HashMap<Position, Vec<String>>>,
}

impl NumberPad {
    fn new() -> Self {
        let mut buttons: HashMap<char, Position> = HashMap::new();
        buttons.insert('7', Position { row: 0, col: 0 });
        buttons.insert('8', Position { row: 0, col: 1 });
        buttons.insert('9', Position { row: 0, col: 2 });
        buttons.insert('4', Position { row: 1, col: 0 });
        buttons.insert('5', Position { row: 1, col: 1 });
        buttons.insert('6', Position { row: 1, col: 2 });
        buttons.insert('1', Position { row: 2, col: 0 });
        buttons.insert('2', Position { row: 2, col: 1 });
        buttons.insert('3', Position { row: 2, col: 2 });
        buttons.insert('X', Position { row: 3, col: 0 });
        buttons.insert('0', Position { row: 3, col: 1 });
        buttons.insert('A', Position { row: 3, col: 2 });

        let excluded_positions: HashSet<Position> = HashSet::from_iter(vec![Position{ row: 3, col: 0 }]);

        return Self { buttons: buttons.clone(), points_map: map_paths(buttons.values().collect::<Vec<&Position>>(), &excluded_positions) };
    }
}

struct DirectionPad {
    buttons: HashMap<char, Position>,
    points_map: HashMap<Position, HashMap<Position, Vec<String>>>,
}

impl DirectionPad {
    fn new() -> Self {
        let mut buttons: HashMap<char, Position> = HashMap::new();
        buttons.insert('X', Position { row: 0, col: 0 });
        buttons.insert('^', Position { row: 0, col: 1 });
        buttons.insert('A', Position { row: 0, col: 2 });
        buttons.insert('<', Position { row: 1, col: 0 });
        buttons.insert('v', Position { row: 1, col: 1 });
        buttons.insert('>', Position { row: 1, col: 2 });

        let excluded_positions: HashSet<Position> = HashSet::from_iter(vec![Position{ row: 0, col: 0 }]);

        return Self { buttons: buttons.clone(), points_map: map_paths(buttons.values().collect::<Vec<&Position>>(), &excluded_positions) };
    }
}

fn map_paths(all_points: Vec<&Position>, excluded_positions: &HashSet<Position>) -> HashMap<Position, HashMap<Position, Vec<String>>> {
    let mut all_paths: HashMap<Position, HashMap<Position, Vec<String>>> = HashMap::new();
    for p1 in all_points.iter() {
        let mut point_paths: HashMap<Position, Vec<String>> = HashMap::new();
        for p2 in all_points.iter() {
            if p1 != p2 {
                point_paths.insert(**p2, get_all_paths(p1, p2, &excluded_positions));
            }
        }
        all_paths.insert(**p1, point_paths);
    }

    return all_paths;
}

fn get_all_paths(position: &Position, target_position: &Position, excluded_positions: &HashSet<Position>) -> Vec<String> {
    let (vertical_delta, horizontal_delta): (i32, i32) = target_position.delta(&position);
    // println!("position = {:?}, target = {:?}, horizontal_delta = {}, vertical_delta = {}", position, target_position, horizontal_delta, vertical_delta);
    let mut path: String = String::new();
    if horizontal_delta > 0 {
        path.push_str(">".repeat(horizontal_delta.abs() as usize).as_str());
    } else {
        path.push_str("<".repeat(horizontal_delta.abs() as usize).as_str());
    }
    if vertical_delta > 0 {
        path.push_str("v".repeat(vertical_delta.abs() as usize).as_str());
    } else {
        path.push_str("^".repeat(vertical_delta.abs() as usize).as_str());
    }

    if path.len() == 0 {
        return vec![path];
    } else {
        let all_paths: HashSet<Vec<char>> = HashSet::from_iter(path.chars().permutations(path.len()));
        return all_paths.iter().map(|x| x.into_iter().collect::<String>()).filter(|x| verify_path(position, x.to_string(), excluded_positions)).collect::<Vec<String>>();
    }
}

fn verify_path(position: &Position, path: String, excluded_positions: &HashSet<Position>) -> bool {
    let mut current_position: Position = position.clone();
    // println!("Verifying position = {:?} with path {:?}", position, path);
    for c in path.chars() {
        current_position = current_position.walk(1, Direction::from(c));
        if excluded_positions.contains(&current_position) {
            return false;
        }
    }
    return true;
}

struct CodeCombinations {
    codes: Vec<Vec<String>>,
    max_codes: usize,
    code_indices: Vec<usize>,
    index: usize,
}

impl CodeCombinations {
    fn new(codes: &Vec<Vec<String>>) -> Self {
        let max_codes: usize = codes.iter().map(|x| x.len()).product();
        let code_indices: Vec<usize> = (0..codes.len()).map(|x| {
            if x == 0 {
                return 1;
            } else {
                return codes[..x].iter().map(|a| a.len()).product();
            }
        }).collect::<Vec<usize>>();

        return Self { codes: codes.clone(), max_codes: max_codes, code_indices: code_indices, index: 0 }
    }
}

impl Iterator for CodeCombinations {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.max_codes {
            return None;
        } else {
            let mut new_code: String = String::new();
            for (i, x) in self.codes.iter().enumerate() {
                new_code.push_str(x[(self.index/self.code_indices[i]) % x.len()].as_str());
            }
            self.index += 1;
            return Some(new_code);
        }
    }
}

fn code_combinations(codes: &Vec<Vec<String>>, ) {
    let codes: Vec<Vec<&str>> = vec![vec!["aa", "bb", "cc"], vec!["dd"], vec!["ee", "ff"]];
    let max_codes: usize = codes.iter().map(|x| x.len()).product();
    let code_index: Vec<usize> = (0..codes.len()).map(|x| {
        if x == 0 {
            return 1;
        } else {
            return codes[..x].iter().map(|a| a.len()).product();
        }
    }).collect::<Vec<usize>>();
    for i in 0..max_codes {
        let code: String = codes.iter().enumerate().map(|(x_index, x)| x[(i/code_index[x_index]) % x.len()]).collect::<String>();
        println!("{}", code);
    }
}

fn get_shortest_code_length(direction_pad: &DirectionPad, depth: usize, position: &Position, code: &String, excluded_positions: &HashSet<Position>, code_lengths: Option<&mut HashMap<(usize, String), usize>>) -> usize {
    // println!("{}Depth = {}: Running get_shortest_code_length on {}", "\t".repeat(4 - depth), depth, code);

    if depth == 0 {
        if code_lengths.is_some() {
            code_lengths.unwrap().insert((depth, code.clone()), code.len());
        }
        return code.len();
    } else {
        let current_code_lengths: &mut HashMap<(usize, String), usize>;
        let mut empty_hashmap: HashMap<(usize, String), usize> = HashMap::new();
        if code_lengths.is_some() {
            current_code_lengths = code_lengths.unwrap();
        } else {
            current_code_lengths = &mut empty_hashmap;
        }

        if current_code_lengths.contains_key(&(depth, code.clone())) {
            return *current_code_lengths.get(&(depth, code.clone())).unwrap();
        }

        let mut current_position: &Position = position;
        let mut all_code_paths: Vec<Vec<String>> = Vec::new();
        let mut total_min_distance: usize = 0;
        let mut target_path: String = String::new();
        for c in code.chars() {
            let target_position: &Position = direction_pad.buttons.get(&c).unwrap();
            let mut paths: Vec<String> = get_all_paths(current_position, target_position, excluded_positions);
            paths.iter_mut().for_each(|x| x.push('A'));
            let mut path_distance: usize = 1000000;
            let mut shortest_path: String = String::new();
            for p in paths.iter() {
                let path_min_distance: usize = get_shortest_code_length(direction_pad, depth - 1, &Position { row: 0, col: 2 }, &p, excluded_positions, Some(current_code_lengths));
                if path_min_distance < path_distance {
                    path_distance = path_min_distance;
                    shortest_path = p.clone();
                }
            }
            total_min_distance += path_distance;
            target_path.push_str(&shortest_path.as_str());
            all_code_paths.push(paths);
            current_position = target_position;
        }

        // println!("{}Shortest path for code {} is {}", "\t".repeat(3 - depth), code, target_path);

        return total_min_distance;
    }
}


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let codes: Vec<Vec<char>> = input_lines.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    
    println!("Building numpad");
    let numpad: NumberPad = NumberPad::new();
    println!("Building dirpad");
    let dirpad: DirectionPad = DirectionPad::new();
    println!("All done");

    for (k,v) in numpad.points_map.clone().into_iter() {
        println!("Point {:?}", k);
        for (xk, xv) in v.into_iter() {
            println!("\tPoint {:?}: {:?}", xk, xv);
        }
    }

    let mut start_position: Position = Position { row: 3, col: 2 };
    let mut code_distances: HashMap<String, usize> = HashMap::new();
    
    for c in codes {
        println!("{:?}", c);
        let mut paths: Vec<String> = Vec::new();
        let mut end_position: Position = *numpad.buttons.get(&c[0]).unwrap();
        paths.append(&mut numpad.points_map.get(&start_position).unwrap().get(&end_position).unwrap().clone());
        paths.iter_mut().for_each(|x| x.push('A'));
        start_position = end_position;
        for x in c[1..].iter() {
            end_position = *numpad.buttons.get(x).unwrap();
            let mut bigger_paths: Vec<String> = Vec::new();
            for p1 in paths {
                for p2 in numpad.points_map.get(&start_position).unwrap().get(&end_position).unwrap().clone() {
                    bigger_paths.push(p1.to_owned() + p2.as_str());
                }
            }
            paths = bigger_paths.clone();
            paths.iter_mut().for_each(|x| x.push('A'));
            start_position = end_position;
        }

        
        let directional_excluded_positions: HashSet<Position> = HashSet::from_iter(vec![Position{row: 0, col: 0}]);
        let mut current_min: usize = 1000000;
        
        for p in paths.iter() {
            let new_min_path: usize = get_shortest_code_length(&dirpad, 2, &Position { row: 0, col: 2}, p, &directional_excluded_positions, None);
            if new_min_path < current_min {
                current_min = new_min_path;
            }
        }
        println!("Code = {:?}, min = {}, Paths = {:?}", c.iter().collect::<String>(), current_min, paths);
        code_distances.insert(c.iter().collect::<String>(), current_min);
    }

    let mut total_complexity: usize = 0;
    for (code, distance) in code_distances {
        let code_numbers: usize = code.replace("A", "").parse::<usize>().unwrap();
        let complexity: usize = code_numbers * distance;
        println!("Code {} has numeric {:3} x distance {}", code, code_numbers, distance);
        total_complexity += complexity;
    }

    return total_complexity;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here

        let path: String = ">^^".to_string();

        for i in 0..4 {
            println!("Doing k = {}", i);
            for p in path.chars().permutations(i) {
                println!("\t{:?}", p)
            }
        }
    }

    #[test]
    fn quick_test2() {
        // Do a quick test here
        let codes: Vec<Vec<&str>> = vec![vec!["aa", "bb", "cc"], vec!["dd"], vec!["ee", "ff"]];
        let max_codes: usize = codes.iter().map(|x| x.len()).product();
        let code_index: Vec<usize> = (0..codes.len()).map(|x| {
            if x == 0 {
                return 1;
            } else {
                return codes[..x].iter().map(|a| a.len()).product();
            }
        }).collect::<Vec<usize>>();
        for i in 0..max_codes {
            let code: String = codes.iter().enumerate().map(|(x_index, x)| x[(i/code_index[x_index]) % x.len()]).collect::<String>();
            println!("{}", code);
        }
    }

    #[test]
    fn quick_test3() {
        // Do a quick test here
        let codes: Vec<Vec<String>> = vec![vec!["aa".to_string(), "bb".to_string(), "cc".to_string()], vec!["dd".to_string()], vec!["ee".to_string(), "ff".to_string()]];
        for x in CodeCombinations::new(&codes) {
            println!("x = {:?}", x);
        }
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 126384);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 94426);
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
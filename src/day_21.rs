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

        println!("Code = {:?}, Paths = {:?}", c.iter().collect::<String>(), paths);
    }


    return 0;
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
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 13);
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
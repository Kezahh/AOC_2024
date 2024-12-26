const INPUTS_FOLDER: &str = "inputs/day_23";

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::generic;


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut computer_map: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input_lines {
        let computer1: String = line[..2].to_string();
        let computer2: String = line[3..].to_string();

        if !computer_map.contains_key(&computer1) {
            computer_map.insert(computer1.clone(), HashSet::new());
        }
        computer_map.get_mut(&computer1).unwrap().insert(computer2.clone());

        if !computer_map.contains_key(&computer2) {
            computer_map.insert(computer2.clone(), HashSet::new());
        }
        computer_map.get_mut(&computer2).unwrap().insert(computer1.clone());

    }

    println!("Map built.");

    let mut connected_computers: HashMap<Vec<String>, bool> = HashMap::new();
    for (c, c_map) in computer_map.iter() {
        // println!("{}: {:?}", c, c_map);
        if c_map.len() >= 2 {
            for permutation in c_map.iter().cloned().permutations(2) {
                let mut attempt_connected: Vec<String> = permutation.clone();
                attempt_connected.push(c.clone());
                attempt_connected.sort();
                // println!("Checking {:?}", attempt_connected);
                if !connected_computers.contains_key(&attempt_connected) {
                    if computer_map.get(&permutation[0]).unwrap().contains(&permutation[1]) && computer_map.get(&permutation[1]).unwrap().contains(&permutation[0]){
                        // println!("\t{}: {:?} contains {}", permutation[0], computer_map.get(&permutation[0]).unwrap(), c);
                        // println!("\t{}: {:?} contains {}", permutation[1], computer_map.get(&permutation[1]).unwrap(), c);
                        connected_computers.insert(attempt_connected, true);
                    } else {
                        connected_computers.insert(attempt_connected, false);
                    }
                }
            }
        }
        // break;
    }
    
    let mut sum_t_lans: usize = 0;
    for (connection, is_connected) in connected_computers {
        if is_connected {
            for computer in connection {
                if computer.starts_with('t') {
                    sum_t_lans += 1;
                    break;
                }
            }
        }
    }
    return sum_t_lans;
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
        assert!(answer == 7);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 1215);
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
const INPUTS_FOLDER: &str = "inputs/day_22";

use std::{collections::{HashMap, HashSet}, ops::BitXor};

use crate::generic;


fn mix(a: usize, b: usize) -> usize {
    return (a as i32).bitxor(b as i32) as usize;
}

fn prune(a: usize) -> usize {
    return a % 16777216;
}

fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let secret_numbers: Vec<usize> = input_lines.iter().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut results: Vec<usize> = Vec::new();
    let mut difference_map: HashMap<Vec<i32>, Vec<usize>> = HashMap::new();
    for s in secret_numbers {
        let mut bananas: Vec<usize> = Vec::new();
        let mut result = s;
        bananas.push(result % 10);
        for _ in 0..2000 {
            result = prune(mix(result * 64, result));
            result = prune(mix(result / 32, result));
            result = prune(mix(result * 2048, result));
            bananas.push(result % 10);
        }
        println!("{}: {}", s, result);
        results.push(result);
        let banana_difference: Vec<i32> = (1..bananas.len()).map(|i| bananas[i] as i32 - bananas[i - 1] as i32).collect::<Vec<i32>>();
        let banana_difference_vec: Vec<Vec<i32>> = (4..banana_difference.len()).map(|i| banana_difference[(i-4)..i].iter().copied().collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
        let mut map_per_secret: HashMap<Vec<i32>, usize> = HashMap::new();
        for (i, difference_vec) in banana_difference_vec.iter().enumerate() {
            let bananas_bought: usize = bananas[i + 4];
            if !map_per_secret.contains_key(difference_vec) {
                map_per_secret.insert(difference_vec.clone(), bananas_bought);
            } else {
                // if bananas_bought > *map_per_secret.get(difference_vec).unwrap() {
                //     map_per_secret.insert(difference_vec.clone(), bananas_bought);
                // }
            }
        }

        for (difference_vec, bananas) in map_per_secret {
            if !difference_map.contains_key(&difference_vec) {
                difference_map.insert(difference_vec.clone(), Vec::new());
            }
            difference_map.get_mut(&difference_vec).unwrap().push(bananas);
        }
    }

    if !part_2 {
        return results.iter().sum::<usize>();
    } else {
        let mut max_bananas: usize = 0;
        for (difference_vec, bananas)  in difference_map {
            let banana_sum = bananas.iter().sum::<usize>();
            if banana_sum > max_bananas {
                max_bananas = banana_sum;
            }

            if bananas.len() > 2 {
                println!("Difference {:?} has {:?}", difference_vec, bananas );
            }
        }

        return max_bananas;
    }
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
        assert!(answer == 37327623);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 14476723788);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_3.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 23);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 1630);

        // 1672 too high
    }
}
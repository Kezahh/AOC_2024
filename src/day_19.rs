const INPUTS_FOLDER: &str = "inputs/day_19";

use std::collections::HashMap;

use crate::generic;


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let towels: Vec<String> = input_lines[0].split(", ").map(|x| x.to_string()).collect::<Vec<String>>();
    let designs: Vec<String> = input_lines[2..].iter().map(|x| x.to_string()).collect::<Vec<String>>();

    let mut sum_good_designs: usize = 0;
    let mut memoisation: HashMap<String, bool> = HashMap::new();

    for design in designs {
        if check_string(design, &towels, &mut memoisation) {
            sum_good_designs += 1;
        }
    }

    return sum_good_designs;
}

fn check_string(design: String, towels: &Vec<String>, memoisation: &mut HashMap<String, bool>) -> bool {
    if design.len() == 0 {
        return true;
    }

    if memoisation.contains_key(&design) {
        return *memoisation.get(&design).unwrap();
    }

    for towel in towels { 
        if design.starts_with(towel) && check_string(design[towel.len()..].to_string(), towels, memoisation) {
            memoisation.insert(design, true);
            return true
        }
    }

    memoisation.insert(design, false);

    return false;
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
        assert!(answer == 6);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 255);
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
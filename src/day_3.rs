const INPUTS_FOLDER: &str = "inputs/day_3";

use std::collections::HashMap;

use crate::generic;
use regex::Regex;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul,
    Do,
    Dont,
}

#[derive(Debug)]
struct Mul {
    x: i32,
    y: i32,
    string_index: usize,
}

impl From<String> for Mul {
    fn from(value: String) -> Self {
        let mut just_numbers: String = value[4..].to_string();
        just_numbers = just_numbers[..just_numbers.len()-1].to_string();
        let split_numbers: Vec<i32> = just_numbers.split(",").map(|x| x.parse::<i32>().expect("Bad number given in input!")).collect::<Vec<i32>>();
        return Self{x: split_numbers[0], y: split_numbers[1], string_index: 0};
    }
}

impl From<&str> for Mul {
    fn from(value: &str) -> Self{
        return Self::from(value.to_string());
    }
}

impl Mul {
    fn multiply(&self) -> i32 {
        return self.x * self.y;
    }

    fn set_string_index(&mut self, value: usize) {
        self.string_index = value;
    }
}


fn solve_puzzle(input_filename: String, part_2: bool) -> i32 {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let full_input: String = input_lines.join("");
    let re_mul = Regex::new(r"mul\([0-9]{1,3}.[0-9]{1,3}\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();
    let mut all_muls: Vec<Mul> = Vec::new();

    let mut all_instructions: HashMap<usize, Instruction> = HashMap::new();

    for x in re_mul.find_iter(&full_input) {
        let mut new_mul = Mul::from(x.as_str());
        new_mul.set_string_index(x.start());
        all_muls.push(new_mul);
        all_instructions.insert(x.start(), Instruction::Mul);
    }

    for x in re_do.find_iter(&full_input) {
        all_instructions.insert(x.start(), Instruction::Do);
    }

    for x in re_dont.find_iter(&full_input) {
        all_instructions.insert(x.start(), Instruction::Dont);
    }
    
    println!("{:?}", all_instructions);
    

    if !part_2 {
        return all_muls.iter().map(|x| x.multiply()).sum();
    } else {
        let mut do_is_on = true;
        let mut mul_index = 0;
        let mut full_sum = 0;
        for start_index in all_instructions.keys().sorted() {

            match all_instructions[start_index] {
                Instruction::Do => do_is_on = true,
                Instruction::Dont => do_is_on = false,
                Instruction::Mul => {
                    if do_is_on {
                        full_sum += all_muls[mul_index].multiply();
                    }
                    mul_index += 1;
                }
            }
        }
        return full_sum
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
        assert!(answer == 161);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 175615763);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_2.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 48);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 74361272);
    }
}
const INPUTS_FOLDER: &str = "inputs/day_17";

use std::{collections::btree_map::Values, ops::BitXor};

use crate::generic;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<usize> for Instruction {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Bad program number given!"),
        }
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        return Instruction::from(value.parse::<usize>().unwrap());
    }
}

impl Instruction {
    fn run(&self, value: usize, register_a: &mut i64, register_b: &mut i64, register_c: &mut i64) -> Option<usize> {
        match self {
            Instruction::Adv => {
                *register_a = *register_a / 2i64.pow(value as u32);
            },
            Instruction::Bxl => {
                *register_b = register_b.bitxor(value as i64);
            },
            Instruction::Bst => {
                *register_b = (value % 8) as i64;
            },
            Instruction::Jnz => {
                if *register_a == 0 {
                    return None;
                } else {
                    // Figure out Jumping
                    return Some(0);
                }
            },
            Instruction::Bxc => {
                
            },
            Instruction::Out => {
                return Some(value % 8);
            },
            Instruction::Bdv => {
                
            },
            Instruction::Cdv => {
                
            },
        }

        return None;
    }
}

fn solve_puzzle(input_filename: String, part_2: bool) -> (String, i64, i64, i64) {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut register_a: i64 = input_lines[0].split(" ").last().unwrap().parse::<i64>().unwrap();
    let mut register_b: i64 = input_lines[1].split(" ").last().unwrap().parse::<i64>().unwrap();
    let mut register_c: i64 = input_lines[2].split(" ").last().unwrap().parse::<i64>().unwrap();
    let program: Vec<usize> = input_lines[4].split(" ").last().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    
    let mut output: Vec<usize> = Vec::new();

    let mut instruction_index: usize = 0;
    let mut instruction_count: usize = 0;

    while instruction_index < program.len() {
        instruction_count += 1;
        let current_instruction: Instruction = Instruction::from(program[instruction_index]);
        let current_literal_operand: usize = program[instruction_index + 1];
        let current_combo_operand: i64 = match current_literal_operand {
            0 | 1 | 2 | 3 => current_literal_operand as i64,
            4 => register_a,
            5 => register_b,
            6 => register_c,
            7 => 7,
            _ => panic!("Bad number in program!"),
        };

        match current_instruction {
            Instruction::Adv => {
                register_a = register_a / 2i64.pow(current_combo_operand as u32);
            },
            Instruction::Bxl => {
                register_b = register_b.bitxor(current_literal_operand as i64);
            },
            Instruction::Bst => {
                register_b = (current_combo_operand % 8) as i64;
            },
            Instruction::Jnz => {
                if register_a != 0 {
                    instruction_index = current_literal_operand;
                }
            },
            Instruction::Bxc => {
                register_b = register_b.bitxor(register_c);
            },
            Instruction::Out => {
                output.push((current_combo_operand % 8) as usize);
            },
            Instruction::Bdv => {
                register_b = register_a / 2i64.pow(current_combo_operand as u32);
            },
            Instruction::Cdv => {
                register_c = register_a / 2i64.pow(current_combo_operand as u32);
            },
        }

        if !(current_instruction == Instruction::Jnz && register_a != 0) {
            instruction_index += 2;
        }
        // println!("A = {}, B = {}, C = {}", register_a, register_b, register_c);

        // if instruction_count == 10 {
        //     break;
        // }
    }


    return (output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","), register_a, register_b, register_c);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here

        let mut register_a: i64 = 0;
        let mut register_b: i64 = 0;
        let mut register_c: i64 = 0;


        register_a = 11;
        Instruction::Adv.run(1, &mut register_a, &mut register_b, &mut register_c);
        assert!(register_a == 5);

        register_a = 11;
        Instruction::Adv.run(2, &mut register_a, &mut register_b, &mut register_c);
        assert!(register_a == 2);

        register_b = 29;
        Instruction::Bxl.run(7, &mut register_a, &mut register_b, &mut register_c);
        assert!(register_b == 26);


        println!("A = {}, B = {}, C = {}", register_a, register_b, register_c);
    }

    #[test]
    fn example_1_1() {
        let (answer, a, b, c) = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1_1.txt", false);
        println!("Answer = {:?}", answer);
        println!("A = {}, B = {}, C = {}", a, b, c);
        assert!(b == 1);
    }

    #[test]
    fn example_1_2() {
        let (answer, a, b, c) = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1_2.txt", false);
        println!("Answer = {:?}", answer);
        println!("A = {}, B = {}, C = {}", a, b, c);
        assert!(answer == "0,1,2");
    }

    #[test]
    fn example_1_3() {
        let (answer, a, b, c) = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1_3.txt", false);
        println!("Answer = {:?}", answer);
        println!("A = {}, B = {}, C = {}", a, b, c);
        assert!(a == 0);
        assert!(answer == "4,2,5,6,7,7,7,7,3,1,0");
    }

    #[test]
    fn example_1_4() {
        let (answer, a, b, c) = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1_4.txt", false);
        println!("Answer = {:?}", answer);
        println!("A = {}, B = {}, C = {}", a, b, c);
        assert!(b == 26);
    }

    #[test]
    fn example_1_5() {
        let (answer, a, b, c) = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1_5.txt", false);
        println!("Answer = {:?}", answer);
        println!("A = {}, B = {}, C = {}", a, b, c);
        assert!(b == 44354);
    }


    #[test]
    fn example_1() {
        let (answer, a, b, c) = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        println!("A = {}, B = {}, C = {}", a, b, c);
        assert!(answer == "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part_1() {
        let (answer, a, b, c) = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        println!("A = {}, B = {}, C = {}", a, b, c);
        assert!(answer == "7,0,3,1,2,6,3,7,1");
    }

    #[test]
    fn example_2() {
        let (answer, a, b, c) = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        println!("A = {}, B = {}, C = {}", a, b, c);
        assert!(answer == "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part_2() {
        let (answer, a, b, c) = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        println!("A = {}, B = {}, C = {}", a, b, c);
        assert!(answer == "4,6,3,5,6,3,5,2,1,0");
    }
}
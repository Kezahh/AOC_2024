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
    let register_a_init: i64 = input_lines[0].split(" ").last().unwrap().parse::<i64>().unwrap();
    let register_b_init: i64 = input_lines[1].split(" ").last().unwrap().parse::<i64>().unwrap();
    let register_c_init: i64 = input_lines[2].split(" ").last().unwrap().parse::<i64>().unwrap();

    let mut register_a: i64 = register_a_init;
    let mut register_b: i64 = register_b_init;
    let mut register_c: i64 = register_c_init;

    let program: Vec<usize> = input_lines[4].split(" ").last().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    
    let mut output_string: String = String::new();
    if !part_2 {
        output_string = do_program(program, &mut register_a, &mut register_b, &mut register_c, part_2);
        return (output_string, register_a, register_b, register_c);
    } else {
        let program_string: String = program.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
        let mut finished: bool = false;
        let mut loop_index: usize = 0;
        let start_offset: i64 = (812262 << 27) + (3 << 24) + (1 << 21) + (2 << 18) + (2 << 15) + (4 << 9) + (2 << 6) + (3 << 3);
        // println!("Start offset {} bits = {:0b}", start_offset.ilog2() + 1, start_offset);
        println!("Program string is {:?}", program_string);
        // let mut loop_index: usize =  35184372000000;
        let total_loops: usize = 29000000 - 3518400;
        // let mut loop_index: usize = 290000000000000;
        // Running loop index 35184663000000 last finished
        // let mut loop_index: usize = 0;
        while !finished {
            let start_a: i64 = (loop_index << 0) as i64 + start_offset;
            register_a = start_a;
            register_b = register_b_init;
            register_c = register_c_init;
            output_string = do_program(program.clone(), &mut register_a, &mut register_b, &mut register_c, part_2);
            
            if output_string == program_string {
                finished = true;
                println!("Register A is {}", start_a);
                //00011000110010011100110011101110001000111100000001 = 2,4,1,5,7,5,0,3,4,1,1,6,5,5,3,0
            }

            if output_string.ends_with("4,1,5,7,5,0,3,4,1,1,6,5,5,3,0") {
                println!("{:050b} = {}", (loop_index << 0) as i64 + start_offset, output_string);
            }

            let mut matches_end: bool = true;
            for j in 0..output_string.len() {
                matches_end = matches_end && output_string.chars().nth_back(j) == program_string.chars().nth(j);
            }

            // if matches_end {
            //     println!("Loop index = {}, output len = {}, end matches", loop_index, output_string.len());
            // }

            if loop_index % 1 == 0 {
                // println!("Running loop index ({}/{})", (loop_index/10000000), total_loops);
                // println!("{:020b} = {}", (loop_index << 14) as i64 + 3841, output_string);
                // println!("\tanswer len = {}", output_string.len());
                // println!("\tprogram_string = {:?}", program_string);
                // println!("\toutput_string = {:?}", output_string);
            }

            if loop_index == 10000000  {
                break;
            }
            loop_index += 1;
        }
    }


    return (output_string, register_a, register_b, register_c);
}

fn do_program(program: Vec<usize>, register_a: &mut i64, register_b: &mut i64, register_c: &mut i64, part_2: bool) -> (String) {
    let mut output: Vec<usize> = Vec::new();
    let mut instruction_index: usize = 0;
    let mut instruction_count: usize = 0;
    let mut output_matches: Vec<bool> = Vec::new();

    while instruction_index < program.len() {
        instruction_count += 1;
        let current_instruction: Instruction = Instruction::from(program[instruction_index]);
        let current_literal_operand: usize = program[instruction_index + 1];
        let current_combo_operand: i64 = match current_literal_operand {
            0 | 1 | 2 | 3 => current_literal_operand as i64,
            4 => *register_a,
            5 => *register_b,
            6 => *register_c,
            7 => 7,
            _ => panic!("Bad number in program!"),
        };

        match current_instruction {
            Instruction::Adv => {
                *register_a = *register_a / 2i64.pow(current_combo_operand as u32);
            },
            Instruction::Bxl => {
                *register_b = register_b.bitxor(current_literal_operand as i64);
            },
            Instruction::Bst => {
                *register_b = (current_combo_operand % 8) as i64;
            },
            Instruction::Jnz => {
                if *register_a != 0 {
                    instruction_index = current_literal_operand;
                }
            },
            Instruction::Bxc => {
                *register_b = register_b.bitxor(*register_c);
            },
            Instruction::Out => {
                output.push((current_combo_operand % 8) as usize);
            },
            Instruction::Bdv => {
                *register_b = *register_a / 2i64.pow(current_combo_operand as u32);
            },
            Instruction::Cdv => {
                *register_c = *register_a / 2i64.pow(current_combo_operand as u32);
            },
        }

        if !(current_instruction == Instruction::Jnz && *register_a != 0) {
            instruction_index += 2;
        }

        // if output_matches.len() < output.len() {
        //     for i in output_matches.len()..output.len() {
        //         if output[i] != program[i] {
        //             return String::new();
        //         } else {
        //             output_matches.push(true);
        //         }
        //     }
        // }

        // println!("A = {}, B = {}, C = {}", register_a, register_b, register_c);

        // if instruction_count == 10 {
        //     break;
        // }
    }

    return output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
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
        let (answer, a, b, c) = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_2.txt", true);
        println!("Answer = {:?}", answer);
        println!("A = {}, B = {}, C = {}", a, b, c);
        assert!(answer == "4,6,3,5,6,3,5,2,1,0");
        // 117440
    }

    #[test]
    fn part_2() {
        let (answer, a, b, c) = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        println!("A = {}, B = {}, C = {}", a, b, c);
        assert!(answer == "4,6,3,5,6,3,5,2,1,0");
        // 109020013201563 answer


        // 25992398 is too low



        // got up to 332,680,000
    }

    #[test]
    fn calc() {
        let loop_index: usize = 25992398;
        let start_offset: usize = (113 << 15) + 3841;
        let register_a: usize = (loop_index << 22) + start_offset;

        println!("Register A = {}", register_a);
        println!("Register A also = {:b}", register_a);
    }
}
const INPUTS_FOLDER: &str = "inputs/day_24";

use std::collections::HashMap;

use itertools::Itertools;

use crate::generic;

enum GateType {
    AND,
    OR,
    XOR,
}

impl From<&str> for GateType {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::AND,
            "OR" => Self::OR,
            "XOR" => Self::XOR,
            _ => panic!("Bad input in GateType::From<T>"),
        }
    }
}

struct Gate {
    inputs: Vec<String>,
    output: String,
    gate_type: GateType,
}

impl Gate {
    fn calc(&self, wires: &mut HashMap<String, usize>) {
        wires.insert(self.output.clone(), match self.gate_type {
            GateType::AND => (self.inputs.iter().map(|x| wires.get(x).unwrap()).sum::<usize>() >= 2) as usize,
            GateType::OR => (self.inputs.iter().map(|x| wires.get(x).unwrap()).sum::<usize>() >= 1) as usize,
            GateType::XOR => (self.inputs.iter().map(|x| wires.get(x).unwrap()).sum::<usize>() == 1) as usize,
        });
    }
}




fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let blank_line_index: usize = input_lines.iter().position(|x| x == "").unwrap();

    let mut all_wires: HashMap<String, usize> = HashMap::new();
    for wire_line in input_lines[..blank_line_index].iter() {
        let mut wire_iterator = wire_line.split(": ");
        let wire_name: String = wire_iterator.next().unwrap().to_string();
        let wire_value: usize = wire_iterator.next().unwrap().parse::<usize>().unwrap();
        all_wires.insert(wire_name, wire_value);
    }

    let mut all_gates: Vec<Gate> = Vec::new();
    for gate_line in input_lines[(blank_line_index + 1)..].iter() {
        let gate_list: Vec<String> = gate_line.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
        all_gates.push(Gate { inputs: vec![gate_list[0].clone(), gate_list[2].clone()], output: gate_list[4].clone(), gate_type: GateType::from(gate_list[1].as_str()) });
    }
    
    println!("{:?}", all_wires);

    while all_gates.len() > 0 {
        let target_gate: Gate = all_gates.remove(0);
        if all_wires.contains_key(&target_gate.inputs[0]) && all_wires.contains_key(&target_gate.inputs[1]) {
            target_gate.calc(&mut all_wires);
        } else {
            all_gates.push(target_gate);
        }
    }

    println!("{:?}", all_wires);

    let mut z_wires: Vec<usize> = Vec::new();
    let mut z_number: usize = 0;
    let mut z_count: usize = 0;
    for wire_name in all_wires.keys().sorted() {
        if wire_name.starts_with("z") {
            let current_z: usize = *all_wires.get(wire_name).unwrap();
            z_wires.push(current_z);
            z_number += current_z << z_count;
            z_count += 1;
        }
    }

    println!("{:?}", z_wires);
    println!("{}", z_number);
    return z_number;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here

        println!("{}", false as usize);
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 2024);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 49574189473968);
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
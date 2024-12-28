const INPUTS_FOLDER: &str = "inputs/day_24";

use std::{collections::{HashMap, HashSet}, ops::{BitAnd, BitXor}};

use itertools::{all, Itertools};

use crate::generic;

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl GateType {
    fn to_string(&self) -> &str {
        match self {
            GateType::AND => "AND",
            GateType::OR => "OR",
            GateType::XOR => "XOR",
        }
    }
}

#[derive(Debug, Clone)]
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

    fn calc_dfs(&self, gates_map: &HashMap<String, Gate>, initial_wires: &HashMap<String, usize>, cheats: &HashMap<usize, usize>, depth: usize) -> usize {
        let first_input: usize;
        if initial_wires.contains_key(&self.inputs[0]) {
            first_input = *initial_wires.get(&self.inputs[0]).unwrap();
        } else {
            first_input = gates_map.get(&self.inputs[0]).unwrap().calc_dfs(gates_map, initial_wires, cheats, depth + 1);
        }

        let second_input: usize;
        if initial_wires.contains_key(&self.inputs[1]) {
            second_input = *initial_wires.get(&self.inputs[1]).unwrap();
        } else {
            second_input = gates_map.get(&self.inputs[1]).unwrap().calc_dfs(gates_map, initial_wires, cheats, depth + 1);
        }

        println!("{}{}", "\t".repeat(depth), self.as_string());
        return match self.gate_type {
            GateType::AND => (first_input + second_input > 1) as usize,
            GateType::OR => (first_input + second_input >= 1) as usize,
            GateType::XOR => (first_input + second_input == 1) as usize,
        };
    }

    fn as_string(&self) -> String {
        return self.inputs[0].to_owned() + " " + self.gate_type.to_string() + " " + self.inputs[1].as_str() + " -> " + self.output.as_str();
    }
}




fn solve_puzzle(input_filename: String, part_2: bool) -> String {
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
        let mut gate_inputs: Vec<String> = vec![gate_list[0].clone(), gate_list[2].clone()];
        gate_inputs.sort();
        all_gates.push(Gate { inputs: gate_inputs, output: gate_list[4].clone(), gate_type: GateType::from(gate_list[1].as_str()) });
    }
    
    println!("{:?}", all_wires);

    if !part_2 {
        all_wires = process_gates(&all_gates, &all_wires);
        return get_number(&all_wires, 'z').to_string();
    } else {
        let x_number: usize = get_number(&all_wires, 'x');
        let y_number: usize = get_number(&all_wires, 'y');
        let target_z: usize = x_number + y_number;
        println!("X = {:b}", x_number);
        println!("Y = {:b}", y_number);
        println!("Z = {:b}", target_z);
        let no_swap_wires = process_gates(&all_gates, &all_wires);
        let no_swap_z = get_number(&no_swap_wires, 'z');
        println!("Z = {:b}", no_swap_z);

        let mut z_gates: Vec<&Gate> = all_gates.iter().filter(|x| x.output.starts_with('z')).collect::<Vec<&Gate>>();
        z_gates.sort_by_key(|x| x.output.clone());
        

        let gates_map: HashMap<String, Gate> = HashMap::from_iter(all_gates.iter().map(|x| (x.output.clone(), x.clone())));
        
        for z_gate in z_gates.iter() {
            // println!("{} is made up from {:?}", z_gate.output, trace_gates(&gates_map, gates_map.get(&z_gate.output).unwrap().clone()));
        }
        

        let mut gate_aliasing: HashMap<String, String> = HashMap::new();
        let mut new_gates: Vec<Gate> = Vec::new();
        for g in all_gates.iter() {
            if g.inputs[0].starts_with("x") || g.inputs[1].starts_with("x") || g.inputs[0].starts_with("y") || g.inputs[1].starts_with("y") {
                let output_alias: String = g.inputs[0].to_owned() + g.gate_type.to_string() + g.inputs[1].as_str();
                gate_aliasing.insert(g.output.clone(), output_alias);
            } else {
                new_gates.push(g.clone());
            }
        }

        // println!("gate_aliasing = {:?}", gate_aliasing);

        for g in new_gates.iter_mut() {
            if gate_aliasing.contains_key(&g.inputs[0]) {
                g.inputs[0] = gate_aliasing.get(&g.inputs[0]).unwrap().clone();
            }
            if gate_aliasing.contains_key(&g.inputs[1]) {
                g.inputs[1] = gate_aliasing.get(&g.inputs[1]).unwrap().clone();
            }
        }

        let mut gates_to_process = new_gates.clone();
        let mut processed_gates: Vec<Gate> = Vec::new();
        let mut loop_count: usize = 0;
        while gates_to_process.len() > 0 {
            // println!("{} gates left", gates_to_process.len());
            gate_aliasing = HashMap::new();
            let mut remaining_gates: Vec<Gate> = Vec::new();
            for g in gates_to_process.iter() {
                if g.inputs[0].len() > 3 && g.inputs[1].len() > 3 {
                    let output_alias: String = "(".to_owned() + g.inputs[0].as_str() + ")" + g.gate_type.to_string() + "(" + g.inputs[1].as_str() + ")";
                    gate_aliasing.insert(g.output.clone(), output_alias);
                    processed_gates.push(g.clone());
                } else {
                    remaining_gates.push(g.clone());
                }
            }

            for g in remaining_gates.iter_mut() {
                if gate_aliasing.contains_key(&g.inputs[0]) {
                    g.inputs[0] = gate_aliasing.get(&g.inputs[0]).unwrap().clone();
                }
                if gate_aliasing.contains_key(&g.inputs[1]) {
                    g.inputs[1] = gate_aliasing.get(&g.inputs[1]).unwrap().clone();
                }
            }

            gates_to_process = remaining_gates.clone();
        }

        for g in processed_gates {
            // println!("{:?}", g);
        }

        println!("Calc Z gates using DFS");
        let mut bit_filter: usize = 1;
        let mut cheats: HashMap<usize, usize> = HashMap::new();
        for i in 0..z_gates.len() {
            let z_number: usize = get_z_number_dfs(i, &z_gates, &gates_map, &all_wires, &cheats);
            println!("{} = {:050b}", z_gates[i].output, z_number);
            if z_number.bitand(bit_filter) != target_z.bitand(bit_filter) {
                println!("Bad bit at index {}", i);
                cheats.insert(i, (z_number >> i).bitxor(1));
                break;
            }

            bit_filter = (bit_filter << 1) + 1;
        }
        println!("Cheats = {:?}", cheats);


        for i in 0..z_gates.len() {
            if gates_map.contains_key(&z_gates[i].inputs[0]) && gates_map.contains_key(&z_gates[i].inputs[1]) {
                let input_gate1: &Gate = gates_map.get(&z_gates[i].inputs[0]).unwrap();
                let input_gate2: &Gate = gates_map.get(&z_gates[i].inputs[1]).unwrap();

                if !gate_is_xy_xor(input_gate1) && !gate_is_xy_xor(input_gate2) {
                    println!("z{:02} is missing X Y XOR input gate.", i);
                }
            } else {
                println!("z{:02} is missing input gates.", i);
            }
        }

        println!("Get z05 DFS");
        z_gates[5].calc_dfs(&gates_map, &all_wires, &cheats, 0);


        // z00 = x00 XOR y00
        // c00 = x00 AND y00
        //
        // z01 = (x01 XOR y01) XOR c00
        // c01 = ((x01 XOR y01) AND c00) OR (x01 AND y01)
        //
        // z02 = (x02 XOR y02) XOR c01
        // c02 = ((x02 XOR y02) AND c01) OR (x02 AND y02)

        let mut xy_gates: Vec<&Gate> = all_gates.iter().filter(|x| x.inputs[0].starts_with("x") && x.inputs[1].starts_with("y")).collect::<Vec<&Gate>>();
        xy_gates.sort_by_key(|x| x.inputs[0].clone());
        let mut xy_XOR_gates: Vec<&Gate> = xy_gates.iter().cloned().filter(|x| x.gate_type == GateType::XOR).collect::<Vec<&Gate>>();
        let mut xy_AND_gates: Vec<&Gate> = xy_gates.iter().cloned().filter(|x| x.gate_type == GateType::AND).collect::<Vec<&Gate>>();

        let xy_XOR_gates_outputs: Vec<String> = xy_XOR_gates.iter().map(|x| x.output.clone()).collect::<Vec<String>>();

        println!("XOR Gates");
        println!("{:?}", xy_XOR_gates);
        println!("AND Gates");
        println!("{:?}", xy_AND_gates);

        let mut carriers: Vec<&Gate> = Vec::new();
        let mut actual_z: Vec<&Gate> = Vec::new();
        carriers.push(xy_AND_gates[0]);
        actual_z.push(xy_XOR_gates[0]);

        // for i in 1..z_gates.len() {
        //     let find_actual_z: Option<Gate> = find_gate(&all_gates, vec![xy_XOR_gates[i].output.clone(), carriers[i-1].output.clone()]);

        // }

        for (i, g) in xy_AND_gates.iter().enumerate() {
            let parent_gates: Vec<&Gate> = all_gates.iter().filter(|x| x.inputs[0] == g.output || x.inputs[1] == g.output).collect::<Vec<&Gate>>();
            if parent_gates.len() != 1 {
                println!("{} Gate {:02} does not have exactly 1 parent. It has {} parents.", g.as_string(), i, parent_gates.len());
            } else {
                // We know this is the carrier gate for i.
                let mut parent_inputs: Vec<String> = parent_gates[0].inputs.clone();
                if parent_inputs[0] == g.output {
                    parent_inputs.remove(0);
                } else if parent_inputs[1] == g.output {
                    parent_inputs.remove(1);
                } else {
                    panic!("Cant fidn the right parent inputs!!!");
                }

                let other_input_gate = gates_map.get(&parent_inputs[0]).unwrap();
                if !xy_XOR_gates_outputs.contains(&other_input_gate.inputs[0]) && !xy_XOR_gates_outputs.contains(&other_input_gate.inputs[1]) {
                    println!("\t\t{}", other_input_gate.as_string());
                }

            }
        }

        for (i, g) in xy_XOR_gates.iter().enumerate() {
            let parent_gates: Vec<&Gate> = all_gates.iter().filter(|x| x.inputs[0] == g.output || x.inputs[1] == g.output).collect::<Vec<&Gate>>();
            if parent_gates.len() != 2 {
                println!("{} Gate {:02} does not have exactly two parents. It has {} parents.", g.as_string(), i, parent_gates.len());
            }
        }

        // Known wrong gates
        // XOR has to take X and Y as input or output a Z.
        // Z can only be output from a XOR.
        // X AND Y only appears twice in the input. Once as a input and once as an output.
        //
        // NOT THIS ONE.... x00 AND y00 -> pgc
        // x06 AND y06 -> z06
        // x10 AND y10 -> nbd
        // y10 XOR x10 -> kbs
        // tsm OR dnc -> z20
        // MAYBE qtf XOR nsp -> ksv
        // MAYBE bnp XOR mtq -> tqq
        // MAYBE hpp XOR cmj -> ckb
        // cmj AND hpp -> z39
        
        
        
        // y20 XOR x20 -> mtq


        let target_outputs: Vec<String> = vec![
            "z06".to_string(), "nbd".to_string(), 
            "kbs".to_string(), "z20".to_string(), "ksv".to_string(),
            "tqq".to_string(), "ckb".to_string(), "z39".to_string()];
        let target_outputs_indices: Vec<usize> = target_outputs.iter().map(|x| all_gates.iter().position(|g| g.output == *x).unwrap()).collect::<Vec<usize>>();
        let mut available_indices: Vec<usize> = (0..all_gates.len()).filter(|x| !target_outputs_indices.contains(x)).collect::<Vec<usize>>();

        let mut finished = false;
        let mut final_answer: String = String::new();
        for combo in available_indices.iter().combinations(1) {
            println!("Running combo {:?}", combo);
            let mut attempt_indices = target_outputs_indices.clone();
            // for x in combo {
            //     attempt_indices.push(*x);
            // }

            // println!("Checking {:?}", attempt_indices);
            
            // for permut in attempt_indices.iter().permutations(attempt_indices.len()) {
            for permut in get_pairs(attempt_indices) {
                // println!("\tstarting with {:?}", permut);
                let mut gate_copy: Vec<Gate> = all_gates.clone();
                swap_gates(&mut gate_copy, &permut);
                let check_wires = process_gates(&gate_copy, &all_wires);
                let check_z_number: usize = get_number(&check_wires, 'z');
                // println!("Z = {:50b}", check_z_number);
                if check_z_number == target_z {
                    println!("Z = {:50b}", check_z_number);
                    println!("Found Z at permut = {:?}", permut);
                    println!("{:?}", permut.iter().map(|x| all_gates[*x].output.clone()).collect::<Vec<String>>());
                    final_answer = permut.iter().map(|x| all_gates[*x].output.clone()).sorted().collect::<Vec<String>>().join(",");
                    finished = true;
                    break;
                }
            }
            // break;
            if finished {
                break;
            }

        }

        // Up to 32 before.
        // started at 10:37pm.
        
        return final_answer;
    }
}

fn get_pairs(target_list: Vec<usize>) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();
    let mut possible_set: HashSet<Vec<Vec<usize>>> = HashSet::new();
    for c1 in target_list.iter().copied().combinations(2) {
        let remaining = target_list.iter().copied().filter(|x| !c1.contains(&x)).collect::<Vec<usize>>();
        for c2 in remaining.iter().copied().combinations(2) {
            let remaining2 = remaining.iter().copied().filter(|x| !c2.contains(&x)).collect::<Vec<usize>>();
            for c3 in remaining2.iter().copied().combinations(2) {
                let mut possibilities: Vec<Vec<usize>> = Vec::new();
                let remaining3 = remaining2.iter().copied().filter(|x| !c3.contains(&x)).collect::<Vec<usize>>();

                possibilities.push(c1.clone());
                possibilities.push(c2.clone());
                possibilities.push(c3);
                possibilities.push(remaining3);
                possibilities.sort();
                if !possible_set.contains(&possibilities) {
                    result.push(possibilities.concat());
                    possible_set.insert(possibilities);
                }
            }
        }
    }

    return result;
}

fn swap_gates(gates: &mut Vec<Gate>, swap_indices: &Vec<usize>) {
    if swap_indices.len() != 8 {
        panic!("Bad amount of swap indices given to swap_gates function");
    }

    let mut temp_output: String;
    temp_output = gates[swap_indices[1]].output.clone();
    gates[swap_indices[1]].output = gates[swap_indices[0]].output.clone();
    gates[swap_indices[0]].output = temp_output;

    temp_output = gates[swap_indices[3]].output.clone();
    gates[swap_indices[3]].output = gates[swap_indices[2]].output.clone();
    gates[swap_indices[2]].output = temp_output;

    temp_output = gates[swap_indices[5]].output.clone();
    gates[swap_indices[5]].output = gates[swap_indices[4]].output.clone();
    gates[swap_indices[4]].output = temp_output;

    temp_output = gates[swap_indices[7]].output.clone();
    gates[swap_indices[7]].output = gates[swap_indices[6]].output.clone();
    gates[swap_indices[6]].output = temp_output;

}

fn find_gate(all_gates: &Vec<Gate>, inputs: Vec<String>) -> Option<Gate> {
    let mut found_gate: Option<Gate> = None;
    for g in all_gates {
        if (g.inputs[0] == inputs[0] && g.inputs[1] == inputs[1] || g.inputs[1] == inputs[0] && g.inputs[0] == inputs[1]) {
            found_gate = Some(g.clone());
        }
    }
    return found_gate
}

fn gate_is_xy_xor(gate: &Gate) -> bool {
    return (gate.gate_type == GateType::XOR && (
        (gate.inputs[0].starts_with("x") && gate.inputs[1].starts_with("y")) ||
        (gate.inputs[1].starts_with("x") && gate.inputs[0].starts_with("y"))
    ));
}

fn get_z_number_dfs(bits: usize, z_gates: &Vec<&Gate>, gates_map: &HashMap<String, Gate>, initial_wires: &HashMap<String, usize>, cheats: &HashMap<usize, usize>) -> usize {
    let mut number: usize = 0;
    let mut count: usize = 0;
    for z in z_gates[..(bits+1)].iter() {
        number += z.calc_dfs(gates_map, initial_wires, cheats, 0) << count;
        count += 1;
    }

    return number;
}

fn trace_gates(gates_map: &HashMap<String, Gate>, start_gate: Gate) -> (String, String) {
    let mut gates_to_follow: Vec<Gate> = Vec::new();
    gates_to_follow.push(start_gate);

    let mut x_input: String = String::new();
    let mut y_input: String = String::new();

    while gates_to_follow.len() > 0 {
        let target_gate = gates_to_follow.remove(0);
        for input_wire in target_gate.inputs {
            if input_wire.starts_with('x') {
                x_input = input_wire;
            } else if input_wire.starts_with('y') {
                y_input = input_wire;
            } else {
                gates_to_follow.push(gates_map.get(&input_wire).unwrap().clone());
            }
        }
        if x_input.len() != 0 && y_input.len() != 0 {
            break;
        }
    }

    return (x_input, y_input);
}

fn process_gates(all_gates: &Vec<Gate>, all_wires: &HashMap<String, usize>) -> HashMap<String, usize> {
    let mut gates: Vec<Gate> = all_gates.clone();
    let mut wires: HashMap<String, usize> = all_wires.clone();
    let mut count: usize = 0;
    let mut previous_gate_len: usize = all_gates.len();
    let mut bad_count: usize = 0;
    let mut gate_count: usize = all_gates.len() * 2;

    while gates.len() > 0 && bad_count < gate_count {
        // println!("Gates = {:?}", gates);
        let target_gate: Gate = gates.remove(0);
        if wires.contains_key(&target_gate.inputs[0]) && wires.contains_key(&target_gate.inputs[1]) {
            target_gate.calc(&mut wires);
        } else {
            gates.push(target_gate);
        }
        count += 1;

        if previous_gate_len == gates.len() {
            // println!("No change to gate len! Exiting early soon.");
            bad_count += 1;
        } else {
            bad_count = 0;
        }

        previous_gate_len = gates.len();
    }
    // println!("Count = {}", count);

    return wires;
} 

fn get_number(all_wires: &HashMap<String, usize>, target_char: char) -> usize {
    let mut number: usize = 0;
    let mut count: usize = 0;
    for wire_name in all_wires.keys().sorted() {
        if wire_name.starts_with(target_char) {
            let current_z: usize = *all_wires.get(wire_name).unwrap();
            number += current_z << count;
            count += 1;
        }
    }

    return number;
}

fn all_pairs(input_vec: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut return_vec: Vec<Vec<usize>> = Vec::new();
    for i in 0..input_vec.len() {
        for j in (i+1)..input_vec.len() {
            return_vec.push(vec![input_vec[i], input_vec[j]]);
        }
    }

    return return_vec;
}


#[cfg(test)]
mod tests {
    use generic::append_to_file;

    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here
        let mut new_vec: Vec<usize> = Vec::new();
        for i in 0..222 {
            new_vec.push(i);
        }

        // let mut permutations: Vec<Vec<&usize>> = new_vec.iter().permutations(8).collect::<Vec<Vec<&usize>>>();
        for (i, p) in new_vec.iter().permutations(8).enumerate() {
            if i % 1000000 == 0 {
                println!("{:9}{:?}", i, p);
            }
        }
        // println!("Len = {}", permutations.len());
    }

    #[test]
    fn quicker_test() {
        let original = vec![1,2,3,4,5,6,7,8];
        for p in get_pairs(original) {
            // println!("{:?}", p);
            append_to_file("inputs\\day_24\\quick_output.txt".to_string(), format!("{:?}", p));
            
        }
        // println!("{}", pairs.len());
    }

    #[test]
    fn quickerer_test() {
        let original = vec![1,2,3,4,5,6,7,8];
        let pairs = all_pairs(&original);
        for p in pairs.iter().combinations(4) {
            // println!("{:?}", p);
            append_to_file("inputs\\day_24\\quick_output2.txt".to_string(), format!("{:?}", p));
            
        }
        // println!("{}", pairs.len());
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == "2024");
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == "49574189473968");
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == "30");
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == "7185540");
    }
}
const INPUTS_FOLDER: &str = "inputs/day_25";

use itertools::all;

use crate::generic;

#[derive(Debug, PartialEq, Eq)]
enum TumblerType {
    Lock,
    Key,
}

#[derive(Debug)]
struct Tumbler {
    tumbler_type: TumblerType,
    columns: Vec<usize>,
}

impl From<Vec<String>> for Tumbler {
    fn from(value: Vec<String>) -> Self {
        let tumbler_type: TumblerType = match value[0].as_str() {
            "#####" => TumblerType::Lock,
            _ => TumblerType::Key,
        };

        let columns: Vec<usize> = (0..value[0].len()).map(|i| value.iter().map(|row| row.chars().nth(i).unwrap()).filter(|x| *x == '#').collect::<Vec<char>>().len() - 1).collect::<Vec<usize>>();

        return Self { tumbler_type: tumbler_type, columns: columns };
    }
}


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut tumblers: Vec<Tumbler> = Vec::new();

    let mut i: usize = 0;
    while i < input_lines.len() {
        tumblers.push(Tumbler::from(input_lines[i..(i+7)].to_vec()));
        i += 8;
    }

    let keys: Vec<&Tumbler> = tumblers.iter().filter(|x| x.tumbler_type == TumblerType::Key).collect::<Vec<&Tumbler>>();
    let locks: Vec<&Tumbler> = tumblers.iter().filter(|x| x.tumbler_type == TumblerType::Lock).collect::<Vec<&Tumbler>>();

    println!("There are {} keys", keys.len());
    println!("There are {} locks", locks.len());

    let mut count_unique_combinations: usize = 0;
    for k in keys.iter() {
        for l in locks.iter() {
            if key_fits_lock(k, l) {
                count_unique_combinations += 1;
            }
        }
    }

    return count_unique_combinations;
}

fn key_fits_lock(key: &Tumbler, lock: &Tumbler) -> bool {
    let mut all_fit: bool = true;
    for i in 0..key.columns.len() {
        if key.columns[i] + lock.columns[i] > 5 {
            all_fit = false;
            break;
        }
    }
    return all_fit;
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
        assert!(answer == 3);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 2993);
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
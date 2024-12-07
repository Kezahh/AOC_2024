const INPUTS_FOLDER: &str = "inputs/day_7";

use std::thread::{self, JoinHandle};

use crate::generic;

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
struct Equation {
    result: i64,
    numbers: Vec<i64>,
    operators: Option<Vec<Operator>>,
}

impl From<&String> for Equation {
    fn from(value: &String) -> Self {
        let split_string = value.split(": ").map(|x| x.to_string()).collect::<Vec<String>>();
        Self {
            result: split_string[0].parse::<i64>().expect(&format!("Bad input given for test value. {}", value)),
            numbers: split_string[1].split(" ").map(|x| x.parse::<i64>().expect(&format!("Bad input given for numbers {}", value))).collect::<Vec<i64>>(),
            operators: None,
        }
    }
}

impl Equation {
    fn is_true(&self, part_2: bool) -> bool {
        // println!("Running is_true (part_2 = {}) for {}: {:?}", part_2, self.result, self.numbers);
        let mut possible_answers: Vec<i64> = Vec::new();
        if self.numbers.len() < 2 {
            return self.numbers[0] == self.result;
        } else {
            possible_answers.append(&mut solve_equation(&self.numbers[0], &self.numbers[1], &self.result, part_2));
            // println!("\tpossible answers = {:?}", possible_answers);
        }

        if possible_answers.contains(&self.result) {
            return true;
        }

        let mut new_answers: Vec<i64> = Vec::new();
        for n in self.numbers[2..].iter() {
            for x in possible_answers.iter() {
                new_answers.append(&mut solve_equation(x, n, &self.result, part_2));
                if new_answers.contains(&self.result) {
                    return true;
                }
            }
            possible_answers = new_answers.clone();
        }
        return false;
    }

    fn is_true_backwards(&self, part_2: bool) -> bool {
        // println!("Running is_true (part_2 = {}) for {}: {:?}", part_2, self.result, self.numbers);
        let mut possible_answers: Vec<i64> = Vec::new();
        let mut reversed_numbers = self.numbers.clone();
        reversed_numbers.reverse();

        if self.numbers.len() < 2 {
            return self.numbers[0] == self.result;
        } else {
            possible_answers.append(&mut solve_equation_backwards(&reversed_numbers[0], &self.result, part_2, None));
            // println!("\tpossible answers = {:?}", possible_answers);
        }

        if possible_answers.contains(&0) || possible_answers.contains(&1) {
            return true;
        }

        let mut new_answers: Vec<i64> = Vec::new();
        let multiplier_per_number: Vec<i64> = self.numbers.iter().map(|x| (10 as i64).pow(x.to_string().len() as u32)).collect::<Vec<i64>>();
        let potential_multipliers: Vec<Vec<i64>> = (1..self.numbers.len())
            .map(|i| (i..self.numbers.len())
                .map(|j| get_product(multiplier_per_number[i..(j+1)].to_vec()))
                .collect::<Vec<i64>>())
            .collect::<Vec<Vec<i64>>>();
        println!("\tnumbers = {:?}", self.numbers);
        println!("\tmultiplier_per_number = {:?}", multiplier_per_number);
        println!("\tpotential multipliers = {:?}", potential_multipliers);
        
        for i in 1..reversed_numbers.len() {
            // println!("\tpossible answers = {:?}", possible_answers);
            // [6, 8, 6, 15]
            // [[10, 100, 10000], [10, 1000], [100], []]

            for x in possible_answers.iter() {
                new_answers.append(&mut solve_equation_backwards(&reversed_numbers[i], x, part_2, Some(potential_multipliers.iter().nth_back(i - 1).unwrap().clone())));
                if new_answers.contains(&0) || new_answers.contains(&1) {
                    return true;
                }
            }
            possible_answers = new_answers.clone();
        }
        return false;
    }
}

fn solve_equation(a: &i64, b: &i64, r: &i64, part_2: bool) -> Vec<i64> {
    let mut new_numbers: Vec<i64> = Vec::new();
    let number_a = a + b;
    let number_b = a * b;

    if number_a <= *r {
        new_numbers.push(number_a);
    }
    if number_b <= *r {
        new_numbers.push(number_b);
    }
    if part_2 {
        let number_c = (a.to_string() + b.to_string().as_str()).parse::<i64>().unwrap();
        if number_c <= *r {
            new_numbers.push(number_c);
        }
    }
    // println!("a = {}, b = {}, new_numbers = {:?}", a, b, new_numbers);
    return new_numbers
}

fn solve_equation_backwards(a: &i64, r: &i64, part_2: bool, multipliers: Option<Vec<i64>>) -> Vec<i64> {
    let mut new_numbers: Vec<i64> = Vec::new();

    let number_a = r - a;
    let number_b = r / a;

    if r - a >= 0 {
        new_numbers.push(r - a);
        if r % a == 0 {
            new_numbers.push(r / a);
        }
    }
    if part_2 && multipliers.is_some() {
        for m in multipliers.unwrap() {
            if r - (a * m) >= 0 {
                new_numbers.push(r - (a * m));
                
                if r % (a * m) == 0 {
                    new_numbers.push(r / (a * m));
                }
            } else {
                break;
            }
        }
    }
    // println!("\tr = {}, a = {:?}, new_numbers = {:?}", r, a, new_numbers);
    return new_numbers
}

fn solve_puzzle(input_filename: String, part_2: bool, threading: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let equations: Vec<Equation> = input_lines.iter().map(|x| Equation::from(x)).collect::<Vec<Equation>>();

    let mut test_value_sum: i64 = 0;

    if !threading
    {
        for (i, e) in equations.iter().enumerate() {
            let equation_passes = e.is_true_backwards(part_2);
            if equation_passes {
                test_value_sum += e.result;
            }
            println!("Equation ({}/850) {:?} is {}", i, e, equation_passes);
        }
    } else {

        let mut handles: Vec<JoinHandle<i64>> = Vec::new();

        for i in 0..17 {
            let equation_slice: Vec<Equation> = equations[(i*50)..((i+1)*50)].to_vec();
            handles.push(thread::spawn( move || {
                let mut thread_test_value_sum: i64 = 0;
                
                for (j, e) in equation_slice.iter().enumerate() {
                    let equation_passes = e.is_true_backwards(part_2);
                    if equation_passes {
                        thread_test_value_sum += e.result;
                    }
                    println!("Thread {} ({}/50):: Equation {:?} is {}", i, j, e, equation_passes);
                }

                return thread_test_value_sum;
            }))
        }

        for h in handles {
            test_value_sum += h.join().unwrap();
        }
    }

    return test_value_sum as usize;
}

fn get_product(numbers: Vec<i64>) -> i64 {
    let mut product: i64 = 1;
    for n in numbers {
        product *= n;
    }
    return product;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here
        // numbers = [11, 6, 16, 20]
        // multiplier_per_number = [100, 10, 100, 100]
        // potential_multipliers = [[10, 1000, 100000], [100, 1000], [100]]
        let numbers = [11, 6, 16, 20];
        println!("numbers: {:?}", numbers);
        let multiplier_per_number: Vec<i64> = numbers.iter().map(|x| (10 as i64).pow(x.to_string().len() as u32)).collect::<Vec<i64>>();
        println!("multiplier per number = {:?}", multiplier_per_number);
        let potential_multipliers: Vec<Vec<i64>> = (1..numbers.len())
            .map(|i| (i..numbers.len()).map(|j| get_product(multiplier_per_number[i..(j+1)].to_vec())).collect::<Vec<i64>>()).collect::<Vec<Vec<i64>>>();
        println!("potential multipliers = {:?}", potential_multipliers);
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false, false);
        println!("Answer = {:?}", answer);
        assert!(answer == 3749);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false, true);
        println!("Answer = {:?}", answer);
        assert!(answer == 5702958180383);
        //5977528110155 too high
    }

    #[test]
    fn string_concat() {
        let a: i64 = 123;
        let b: i64 = 556;
        let new_number = (a.to_string() + b.to_string().as_str()).parse::<i64>().unwrap();
        assert!(new_number == 123556);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true, false);
        println!("Answer = {:?}", answer);
        assert!(answer == 11387);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true, false);
        println!("Answer = {:?}", answer);
        assert!(answer == 7185540);
    }
}
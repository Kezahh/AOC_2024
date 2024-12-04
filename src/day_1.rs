const INPUTS_FOLDER: &str = "inputs/day_1";

use std::collections::HashMap;

use crate::generic;


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut left_list: Vec<i32> = input_lines.iter().map(|x| x.split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut right_list: Vec<i32> = input_lines.iter().map(|x| x.split(" ").collect::<Vec<&str>>().last().unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    left_list.sort();
    right_list.sort();

    let answer: i32;

    if !part_2
    {
        answer = left_list.iter().enumerate().map(|(i, x)| (x - right_list[i]).abs()).sum::<i32>();
    }
    else {
        let mut right_counts: HashMap<i32, usize> = HashMap::new();
        for x in right_list {
            let number = right_counts.entry(x).or_insert(0);
            *number += 1;
        }

        answer = left_list.iter().map(|x| x * (*right_counts.get(x).unwrap_or(&0) as i32)).sum::<i32>();
    }
    



    // println!("{:?}", left_list);
    // println!("{:?}", right_list);

    return answer as usize;
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
        assert!(answer == 11);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 2769675);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 31);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 24643097);
    }
}
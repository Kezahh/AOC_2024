const INPUTS_FOLDER: &str = "inputs/day_2";

use itertools::Itertools;

use crate::generic;

fn report_is_safe(report: &Vec<usize>, ignore_count: usize) -> bool {
    if ignore_count == 0 {
        // println!("Checking report {:?}", report);
        let mut increases: bool = true;
        let mut decreases: bool = true;
        for i in 1..report.len() {
            let difference: i32 = report[i] as i32 - report[i - 1] as i32;
            if difference.abs() < 1 || difference.abs() > 3 {
                // Difference up or down must be 1 <= x <= 3.
                return false;
            }

            increases = increases & (difference > 0);
            decreases = decreases & (difference < 0);

            if !increases && !decreases {
                return false;
            }
        }
        return true;
    } else {
        for combo in (0..report.len()).combinations(ignore_count) {
            let mut local_report: Vec<usize> = Vec::new();
            for i in 0..report.len() {
                if !combo.contains(&i) {
                    local_report.push(report[i]);
                }
            }
            if report_is_safe(&local_report, 0) {
                return true;
            }
        }
        return report_is_safe(report, 0);
    }
}

fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let reports: Vec<Vec<usize>> = input_lines.iter().map(|x| x.split(" ").map(|y| y.parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>();

    if !part_2 {
        return reports.iter().filter(|x| report_is_safe(x, 0)).collect::<Vec<&Vec<usize>>>().len();
    } else {
        return reports.iter().filter(|x| report_is_safe(x, 1)).collect::<Vec<&Vec<usize>>>().len();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here
        let report_strings: Vec<&str> = vec![
            "90 91 93 96 93",
            "3 5 7 10 11 11",
            "35 37 39 42 46",
            "67 70 72 74 79",
            "9 12 13 16 15 16 19",
            "48 51 52 55 58 61 58 57",
            "3 4 7 9 8 9 9",
            "22 25 28 30 28 32",
            "38 41 44 45 42 49",
            "54 57 59 59 61"
        ];

        let reports: Vec<Vec<usize>> = report_strings.iter().map(|x| x.split(" ").map(|y| y.parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>();

        assert!(report_is_safe(&reports[0], 1) == true);
        assert!(report_is_safe(&reports[1], 1) == true);
        assert!(report_is_safe(&reports[2], 1) == true);
        assert!(report_is_safe(&reports[3], 1) == true);
        assert!(report_is_safe(&reports[4], 1) == true);
        assert!(report_is_safe(&reports[5], 1) == false);
        assert!(report_is_safe(&reports[6], 1) == false);
        assert!(report_is_safe(&reports[7], 1) == true);
        assert!(report_is_safe(&reports[8], 1) == false);
        assert!(report_is_safe(&reports[9], 1) == true);
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 2);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 421);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 4);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 476);
    }
}
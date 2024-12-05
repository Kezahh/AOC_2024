const INPUTS_FOLDER: &str = "inputs/day_2";

use crate::generic;

#[derive(Debug, PartialEq)]
enum ReportSafety {
    Safe,
    Unsafe,
}

#[derive(Debug, Clone)]
struct Report {
    levels: Vec<usize>,
}

impl From<&String> for Report {
    fn from(value: &String) -> Self {
        return Self{levels: value.split(" ").map(|x| x.parse::<usize>().expect("Bad integer given")).collect::<Vec<usize>>()};
    }
}

impl Report {
    fn safety(&self) -> ReportSafety {
        let mut increases: bool = true;
        let mut decreases: bool = true;
        let mut difference: i32;
        for i in 1..self.levels.len() {
            difference = self.levels[i] as i32 - self.levels[i - 1] as i32;
            if difference.abs() < 1 || difference.abs() > 3 {
                return ReportSafety::Unsafe;
            }

            increases = increases & (difference > 0);
            decreases = decreases & (difference < 0);

            if !increases && !decreases {
                return ReportSafety::Unsafe;
            }
        }

        return ReportSafety::Safe;
    }

    fn remove_level(&self, target_level_index: usize) -> Self {
        let mut new_report: Self = self.clone();
        new_report.levels.remove(target_level_index);

        println!("old report = {:?}", self);
        println!("new report = {:?}", new_report);

        return new_report;
    }

    fn safety_part2(&self) -> ReportSafety {
        let mut increases: bool = true;
        let mut increases_1_problem: bool = false;
        let mut decreases: bool = true;
        let mut decreases_1_problem: bool = false;
        let mut difference: i32;
        let mut difference_1_problem: bool = false;
        
        println!("Running {:?}", self);

        for i in 1..self.levels.len() {
            difference = self.levels[i] as i32 - self.levels[i - 1] as i32;
            if difference.abs() < 1 || difference.abs() > 3 {
                match self.remove_level(i).safety() {
                    ReportSafety::Safe => return ReportSafety::Safe,
                    ReportSafety::Unsafe => return self.remove_level(i - 1).safety(),
                }
            }

            if difference > 0 {
                if decreases {
                    match self.remove_level(i).safety() {
                        ReportSafety::Safe => return ReportSafety::Safe,
                        ReportSafety::Unsafe => match self.remove_level(i - 1).safety() {
                            ReportSafety::Safe => return ReportSafety::Safe,
                            ReportSafety::Unsafe => decreases = false,
                        }
                    }
                }
            } else if difference < 0 {
                if increases {
                    match self.remove_level(i).safety() {
                        ReportSafety::Safe => return ReportSafety::Safe,
                        ReportSafety::Unsafe => match self.remove_level(i - 1).safety() {
                            ReportSafety::Safe => return ReportSafety::Safe,
                            ReportSafety::Unsafe => increases = false,
                        }
                    }
                }
            }

            println!("\tdifference={}", difference);
            println!("\tincreases={}", increases);
            println!("\tincreases_1_problem={}", increases_1_problem);
            println!("\tdecreases={}", decreases);
            println!("\tdecreases_1_problem={}", decreases_1_problem);

            

            if !increases && !decreases {
                return ReportSafety::Unsafe;
            }
        }

        return ReportSafety::Safe;
    }
}



fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());

    let reports: Vec<Report> = input_lines.iter().map(|x| Report::from(x)).collect::<Vec<Report>>();

    for r in reports.iter() {
        if !part_2 {
            println!("{:?}: {:?}", r, r.safety());
        } else {
            println!("{:?}: {:?}", r, r.safety_part2());
        }
    }

    if !part_2 {
        return reports.iter().filter(|x| x.safety() == ReportSafety::Safe).collect::<Vec<&Report>>().len();
    } else {
        return reports.iter().filter(|x| x.safety_part2() == ReportSafety::Safe).collect::<Vec<&Report>>().len();
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

        let reports = report_strings.iter().map(|x| Report::from(&x.to_string())).collect::<Vec<Report>>();

        assert!(reports[0].safety_part2() == ReportSafety::Safe);
        assert!(reports[1].safety_part2() == ReportSafety::Safe);
        assert!(reports[2].safety_part2() == ReportSafety::Safe);
        assert!(reports[3].safety_part2() == ReportSafety::Safe);
        assert!(reports[4].safety_part2() == ReportSafety::Safe);
        assert!(reports[5].safety_part2() == ReportSafety::Unsafe);
        assert!(reports[6].safety_part2() == ReportSafety::Unsafe);
        assert!(reports[7].safety_part2() == ReportSafety::Safe);
        assert!(reports[8].safety_part2() == ReportSafety::Unsafe);
        assert!(reports[9].safety_part2() == ReportSafety::Safe);
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
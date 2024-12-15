const INPUTS_FOLDER: &str = "inputs/day_14";

use std::collections::HashSet;

use crate::generic::{self, append_to_file};
use crate::generic::{Position, Direction};

#[derive(Debug)]
struct Robot {
    start: Position,
    dx: i32,
    dy: i32,
}

impl From<&String> for Robot {
    fn from(value: &String) -> Self {
        let position: Vec<&str> = value.split(" ").collect::<Vec<&str>>()[0][2..].split(",").collect::<Vec<&str>>();
        let velocity: Vec<&str> = value.split(" ").collect::<Vec<&str>>()[1][2..].split(",").collect::<Vec<&str>>();

        return Self { start: Position{row: position[1].parse::<usize>().unwrap(), col: position[0].parse::<usize>().unwrap()},
            dx: velocity[0].parse::<i32>().unwrap(), dy: velocity[1].parse::<i32>().unwrap() };
    }
}

impl Robot {
    fn walk(&self, steps: usize, max_width: usize, max_height: usize) -> Position {
        // println!("Robot = {:?}", self);
        // println!("Robot walking from {:?}", self.start);
        let mut new_row = (self.start.row as i32 + (steps as i32 * self.dy)) % max_height as i32;
        let mut new_col = (self.start.col as i32 + (steps as i32 * self.dx)) % max_width as i32;
        if new_row < 0 {
            new_row += max_height as i32;
        }
        if new_col < 0 {
            new_col += max_width as i32;
        }
        // println!("new_row = {}", new_row);
        // println!("new_col = {}", new_col);
        return Position { row: new_row as usize, col: new_col as usize };
    }

    fn walk_modify(&mut self, steps: usize, max_width: usize, max_height: usize) {
        self.start = self.walk(1, max_width, max_height);
    }
}

fn solve_puzzle(input_filename: String, part_2: bool, width: usize, height: usize) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut robots: Vec<Robot> = input_lines.iter().map(|x| Robot::from(x)).collect::<Vec<Robot>>();

    
    let mid_width: usize = (width / 2);
    let mid_height: usize = (height / 2);
    let mut blank_map: Vec<Vec<char>> = vec![vec!['.'; width]; height];
    
    if !part_2 {
        let mut robot_end_positions: Vec<Position> = robots.iter().map(|x| x.walk(100, width, height)).collect::<Vec<Position>>();

        let mut q_top_left: usize = 0;
        let mut q_top_right: usize = 0;
        let mut q_bot_left: usize = 0;
        let mut q_bot_right: usize = 0;

        println!("Mid with = {}, mid_height = {}", mid_width, mid_height);

        for p in robot_end_positions {
            blank_map[p.row][p.col] = 'X';
            println!("\tPoint = {:?}", p);
            if p.row < mid_height && p.col < mid_width {
                // println!("\t\tAdd to q_top_left");
                q_top_left += 1;
            } else if p.row < mid_height && p.col > mid_width {
                // println!("\t\tAdd to q_top_right");
                q_top_right += 1;
            } else if p.row > mid_height && p.col < mid_width {
                // println!("\t\tAdd to q_bot_left");
                q_bot_left += 1;
            } else if p.row > mid_height && p.col > mid_width {
                // println!("\t\tAdd to q_bot_right");
                q_bot_right += 1;
            }
        }

        println!("q_top_left = {}", q_top_left);
        println!("q_top_right = {}", q_top_right);
        println!("q_bot_left = {}", q_bot_left);
        println!("q_bot_right = {}", q_bot_right);

        for r in blank_map {
            for c in r {
                print!("{}", c);
            }
            println!("");
        }

        return q_top_left * q_top_right * q_bot_left * q_bot_right;
    } else {
        let mut i: usize = 0;
        let mut found_tree: bool = false;
        while (!found_tree) {
            i += 1;
            if i % 1000 == 0 {
                println!("looking for tree after {} seconds", i);
            }
            robots.iter_mut().for_each(|x| x.walk_modify(1, width, height));
            let all_points: HashSet<Position> = HashSet::from_iter(robots.iter().map(|x| x.start));

            // write_map(&all_points, width, height);

            // let average_row: i32 = robots.iter().map(|x| x.start.row).sum::<usize>() as i32/ robots.len() as i32;
            // let average_col: i32 = robots.iter().map(|x| x.start.col).sum::<usize>() as i32/ robots.len() as i32;

            // let average_d_row: f32 = robots.iter().map(|x| (x.start.row as i32 - average_row).pow(2)).sum::<i32>() as f32 / (robots.len() as f32);
            // let average_d_col: f32 = robots.iter().map(|x| (x.start.col as i32 - average_col).pow(2)).sum::<i32>() as f32 / (robots.len() as f32);
            // let output_string: String = format!("{},{},{},{},{}", i, average_row, average_col, average_d_row, average_d_col);
            // append_to_file(INPUTS_FOLDER.to_owned() + "/output.txt", output_string);

            // if all_points.len() == robots.len() {
            //     // all robots have individual spots.
            //     for p in all_points.iter() {
            //         blank_map[p.row][p.col] = 'X';
            //     }

            //     for r in blank_map.iter() {
            //         for c in r {
            //             print!("{}", c);
            //         }
            //         println!("");
            //     }

            //     found_tree = true;
            //     println!("found tree at seconds = {}", i);
            //     break;
            // }
            // println!("{:?}", all_points);
            let mid_col: HashSet<&Position> = HashSet::from_iter(all_points.iter().filter(|x| x.col > mid_width-10 && x.col < mid_width + 10).collect::<Vec<&Position>>());
            for x in mid_col {
                let tree_top: HashSet<Position> = make_tree_top(x);
                // println!("\ttree top = {:?}", tree_top);
                if tree_top.intersection(&all_points).collect::<Vec<&Position>>().len() == 9 {
                    for p in all_points.iter() {
                        blank_map[p.row][p.col] = 'X';
                    }

                    for r in blank_map.iter() {
                        for c in r {
                            print!("{}", c);
                        }
                        println!("");
                    }

                    found_tree = true;
                    println!("found tree at seconds = {}", i);
                    break;
                }
            }

            if i > 10000 {
                break;
            }
        }

        return i;
    }
}

fn write_map(all_points: &HashSet<Position>, width: usize, height: usize) {
    let mut blank_map: Vec<Vec<char>> = vec![vec!['.'; width]; height];
    for p in all_points.iter() {
        blank_map[p.row][p.col] = 'X';
    }

    for r in blank_map.iter() {
        let mut row_string: String = String::new();
        for c in r {
            row_string.push(*c);
        }
        append_to_file(INPUTS_FOLDER.to_owned() + "/output2.txt", row_string);
    }
}

fn make_tree_top(start: &Position) -> HashSet<Position> {
    let mut return_hash: HashSet<Position> = HashSet::new();
    return_hash.insert(*start);
    for i in 1..5 {
        return_hash.insert(Position { row: start.row + i, col: start.col - i });
        return_hash.insert(Position { row: start.row + i, col: start.col + i });
    }

    return return_hash;
}


#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here
        let robot: Robot = Robot { start: Position { row: 4, col: 2 }, dx: 2, dy: -3 };
        println!("{:?}", robot.walk(5, 11, 7));
        println!("{}", 5 % 7);
        println!("{}", -9 % 7);
        println!("{}", 101/2);
    }

    #[test]
    fn quick_test2() {
        let h1: HashSet<usize> = HashSet::from_iter(vec![1,2,3,4]);
        let h2: HashSet<usize> = HashSet::from_iter(vec![1,2,3,4,5,6,7,8,9,10]);

        println!("{:?}", h1.intersection(&h2).collect::<Vec<&usize>>().len());
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false, 11, 7);
        println!("Answer = {:?}", answer);
        assert!(answer == 12);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false, 101, 103);
        println!("Answer = {:?}", answer);
        assert!(answer == 224438715);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_3.txt", true, 103, 101);
        println!("Answer = {:?}", answer);
        assert!(answer == 30);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true, 101, 103);
        println!("Answer = {:?}", answer);
        assert!(answer == 7603);
    }
}
const INPUTS_FOLDER: &str = "inputs/day_18";

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::generic;
use crate::generic::Position;

enum MemoryType {
    Free,
    Corrupted,
}

fn solve_puzzle(input_filename: String, part_2: bool, grid_size: usize, bytes_fallen: usize) -> String {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let corrupt_points: Vec<Position> = input_lines.iter().map(|x| {
        let coordinates: Vec<usize> = x.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        return Position{ row: coordinates[1], col: coordinates[0] };
    }).collect::<Vec<Position>>();

    let all_points: Vec<Position> = (0..grid_size).map(|r| (0..grid_size).map(|c| Position{row: r, col: c}).collect::<Vec<Position>>()).concat();
    let all_points_hash: HashSet<Position> = HashSet::from_iter(all_points);
    let corrupt_points_hash: HashSet<Position> = HashSet::from_iter(corrupt_points[..bytes_fallen].iter().copied());

    let free_points: Vec<Position> = all_points_hash.difference(&corrupt_points_hash).copied().collect_vec();
    let start_position: Position = Position { row: 0, col: 0 };
    let end_position: Position = Position { row: grid_size - 1, col: grid_size - 1 };

    if !part_2 {
        let distance_to_end: usize = djikstra(grid_size, start_position, end_position, corrupt_points_hash).unwrap();
        return distance_to_end.to_string();
    } else {
        let mut finished: bool = false;
        let mut byte_index: usize = 0;
        while !finished {
            let corrupt_points_hash_copy: HashSet<Position> = HashSet::from_iter(corrupt_points[0..byte_index].iter().copied());
            let distance_to_end: Option<usize> = djikstra(grid_size, start_position, end_position, corrupt_points_hash_copy);
            if distance_to_end.is_none() {
                finished = true;
            } else {
                byte_index += 1
            };
        }
        return format!("{},{}", corrupt_points[byte_index - 1].col, corrupt_points[byte_index - 1].row);
    }
}

fn djikstra(grid_size: usize, start_position: Position, end_position: Position, corrupt_points_hash: HashSet<Position>) -> Option<usize> {
    let mut point_distances_to_start: HashMap<Position, usize> = HashMap::new();
    let mut points_to_do: Vec<Position> = vec![start_position];
    let mut done_points: HashSet<Position> = HashSet::new();

    while points_to_do.len() > 0 {
        // println!("distances = {:?}", point_distances_to_start);
        let current_point: Position = points_to_do.remove(0);

        if !done_points.contains(&current_point) {
            done_points.insert(current_point);

            let neighbours = current_point.get_neighbours(grid_size, grid_size);

            // println!("current_point = {:?}, neighbours = {:?}", current_point, neighbours);

            for n in neighbours.iter().copied().filter(|x| !corrupt_points_hash.contains(&x)) {
                if current_point == start_position {
                    point_distances_to_start.insert(current_point, 0);
                    point_distances_to_start.insert(n, 1);
                    if !points_to_do.contains(&n) {
                        points_to_do.push(n);
                    }
                    done_points.remove(&n);
                } else {
                    let distance_to_current: usize = *point_distances_to_start.get(&current_point).unwrap();
                    if point_distances_to_start.contains_key(&n) {
                        let distance_to_neighbour: usize = *point_distances_to_start.get(&n).unwrap();
                        if distance_to_current + 1 < distance_to_neighbour {
                            point_distances_to_start.insert(n, distance_to_current + 1);
                            if !points_to_do.contains(&n) {
                                points_to_do.push(n);
                            }
                            done_points.remove(&n);
                        }
                    } else {
                        point_distances_to_start.insert(n, distance_to_current + 1);
                        if !points_to_do.contains(&n) {
                            points_to_do.push(n);
                        }
                    }
                }
            }
        }
    }

    // print_map(grid_size, &point_distances_to_start);

    return point_distances_to_start.get(&end_position).copied();
}

fn print_map(grid_size: usize, point_distances_to_start: &HashMap<Position, usize>) {
    for r in 0..grid_size {
        for c in 0..grid_size {
            let quick_position: Position = Position { row: r, col: c };
            if !point_distances_to_start.contains_key(&quick_position) {
                print!("     ");
            } else {
                print!("{:5}", point_distances_to_start.get(&quick_position).unwrap());
            }
            print!(" ");
        }
        print!("\n");
    }
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
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false, 7, 12);
        println!("Answer = {:?}", answer);
        assert!(answer == "22");
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false, 71, 1024);
        println!("Answer = {:?}", answer);
        assert!(answer == "284");
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true, 7, 12);
        println!("Answer = {:?}", answer);
        assert!(answer == "6,1");
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true, 71, 1024);
        println!("Answer = {:?}", answer);
        assert!(answer == "51,50");
    }
}
const INPUTS_FOLDER: &str = "inputs/day_11";

use std::collections::HashMap;

use crate::generic;


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut stones: Vec<usize> = input_lines[0].split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut new_stones: Vec<usize> = Vec::new();

    let blink_count: usize;

    if !part_2 {
        blink_count = 25;
    } else {
        blink_count = 75;
    }

    let mut previous_stones: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut previous_stones_by_5: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut stone_lengths: HashMap<usize, HashMap<usize, usize>> = HashMap::new();

    let mut stone_sum: usize = 0;

    for s in stones.iter() {
        // new_stones.append(&mut blink_stone(s.clone(), blink_count, &mut previous_stones, &mut previous_stones_by_5, &mut stone_lengths));
        stone_sum += blink_length(s.clone(), blink_count, &mut stone_lengths);
    }

    return stone_sum;
}

fn blink_length(stone: usize, blink_depth: usize, previous_stones: &mut HashMap<usize, HashMap<usize, usize>>) -> usize {
    if blink_depth == 0 {
        // println!("previous stones = {:?}", previous_stones);
        return 1;
    }

    let mut return_length: usize = 0;
    
    if !previous_stones.contains_key(&stone) {
        previous_stones.insert(stone, HashMap::new());
    }
    
    let stone_hash: &HashMap<usize, usize> = previous_stones.get(&stone).unwrap();

    if stone_hash.contains_key(&blink_depth) {
        return stone_hash.get(&blink_depth).unwrap().clone();
    } else {
        let mut new_stone: usize;
        if stone == 0 {
            new_stone = 1;
            return_length = blink_length(new_stone, blink_depth - 1, previous_stones);
            previous_stones.get_mut(&stone).unwrap().insert(blink_depth, return_length);
            return return_length;
        } else {
            let stone_digits: usize = stone.ilog10() as usize + 1;
            if stone_digits % 2 == 0 {
                let new_stone1 = stone / 10usize.pow(stone_digits as u32/2);
                let new_stone2 = stone % 10usize.pow(stone_digits as u32/2);

                return_length = blink_length(new_stone1, blink_depth - 1, previous_stones) + blink_length(new_stone2, blink_depth - 1, previous_stones);
                previous_stones.get_mut(&stone).unwrap().insert(blink_depth, return_length);
                return return_length;
            } else {
                new_stone = stone * 2024;
                return_length = blink_length(new_stone, blink_depth - 1, previous_stones);
                previous_stones.get_mut(&stone).unwrap().insert(blink_depth, return_length);
                return return_length;
            }
        }
    }
}




fn blink_stone(stone: usize, blink_depth: usize, previous_stones: &mut HashMap<usize, Vec<usize>>, previous_stones_by_5: &mut HashMap<usize, Vec<usize>>, stone_lengths: &mut HashMap<usize, HashMap<usize, usize>>) -> Vec<usize> {
    if blink_depth == 0 {
        println!("returning a stone, previous_stones.len = {}", previous_stones.len());
        println!("returning a stone, previous_stones_by_5.len = {}", previous_stones_by_5.len());
        // println!("previous_stones = {:?}", previous_stones);
        return vec![stone];
    }
    
    let mut new_stones: Vec<usize> = Vec::new();

    if blink_depth > 10 && !previous_stones_by_5.contains_key(&stone) {
        let stone_in_5 = blink_stone(stone, 10, previous_stones, previous_stones_by_5, stone_lengths);
        previous_stones_by_5.insert(stone, stone_in_5);
    }

    if blink_depth > 10 && previous_stones_by_5.contains_key(&stone) {
        for s in previous_stones_by_5.get(&stone).unwrap().clone() {
            new_stones.append(&mut blink_stone(s, blink_depth - 10, previous_stones, previous_stones_by_5, stone_lengths));
        }
    } else {
        if previous_stones.contains_key(&stone) {
            for s in previous_stones.get(&stone).unwrap().clone() {
                new_stones.append(&mut blink_stone(s, blink_depth - 1, previous_stones, previous_stones_by_5, stone_lengths));
            }
        } else {
            let mut new_stone: usize;
            if stone == 0 {
                new_stone = 1;
                previous_stones.insert(stone, vec![new_stone]);
                new_stones.append(&mut blink_stone(new_stone, blink_depth - 1, previous_stones, previous_stones_by_5, stone_lengths));
            } else {
                let stone_digits: usize = stone.ilog10() as usize + 1;
                if stone_digits % 2 == 0 {
                    let new_stone1 = stone / 10usize.pow(stone_digits as u32/2);
                    let new_stone2 = stone % 10usize.pow(stone_digits as u32/2);
                    
                    previous_stones.insert(stone, vec![new_stone1, new_stone2]);

                    new_stones.append(&mut blink_stone(new_stone1, blink_depth - 1, previous_stones, previous_stones_by_5, stone_lengths));
                    new_stones.append(&mut blink_stone(new_stone2, blink_depth - 1, previous_stones, previous_stones_by_5, stone_lengths));
                } else {
                    new_stone = stone * 2024;
                    previous_stones.insert(stone, vec![new_stone]);
                    new_stones.append(&mut blink_stone(new_stone, blink_depth - 1, previous_stones, previous_stones_by_5, stone_lengths));
                }
            }
        }
    }

    return new_stones;
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
        assert!(answer == 55312);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 194782);
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
        assert!(answer == 233007586663131);
    }
}
const INPUTS_FOLDER: &str = "inputs/day_9";

use std::fmt;

use itertools::rev;

use crate::generic;

#[derive(PartialEq, Eq, Clone, Debug)]
enum FileType {
    File,
    Free
}

#[derive(PartialEq, Eq, Clone)]
struct FileBlock {
    file_index: usize,
    file_size: usize,
    file_type: FileType,
}

impl fmt::Debug for FileBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.file_type {
            FileType::File => write!(f, "{}", self.file_index.to_string().repeat(self.file_size)),
            FileType::Free => write!(f, "{}", ".".to_string().repeat(self.file_size)),
        }
    }
}


#[derive(Debug)]
struct DiskMap {
    file_lengths: Vec<usize>,
    free_lengths: Vec<usize>,
    file_lengths_bak: Vec<usize>,
    current_file_index: usize,
    current_free_index: usize,
    current_file_index_bak: usize,
    current_file_size_remaining: usize,
    current_free_size_remaining: usize,
    current_file_size_remaining_bak: usize,
    doing_file: bool,
    iterations: usize,
    total_sum: usize,
}

impl Iterator for DiskMap {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iterations >= self.total_sum {
            return None;
        }

        if self.current_file_size_remaining == 0 && self.doing_file {
            self.current_file_index += 1;
            if self.current_file_index > self.file_lengths.len() {
                if self.current_free_index > self.free_lengths.len() {
                    return None;
                } else {
                    self.doing_file = !self.doing_file;
                }
            } else {
                self.current_file_size_remaining = self.file_lengths[self.current_file_index - 1];
            }
        } else if self.current_free_size_remaining == 0  && !self.doing_file {
            self.current_free_index += 1;
            if self.current_free_index > self.free_lengths.len() {
                if self.current_file_index > self.file_lengths.len() {
                    return None;
                } else {
                    self.doing_file = !self.doing_file;
                }
            } else {
                self.current_free_size_remaining = self.free_lengths[self.current_free_index - 1];
            }
        }

        if self.current_free_size_remaining > 0 {
            self.current_free_size_remaining -= 1;
            if self.current_free_size_remaining == 0 {
                self.doing_file = !self.doing_file;
            }
            self.current_file_size_remaining_bak -= 1;
            let return_char: usize = self.file_lengths.len() - self.current_file_index_bak - 1;
            if self.current_file_size_remaining_bak == 0 {
                self.current_file_index_bak += 1;
                self.current_file_size_remaining_bak = self.file_lengths_bak[self.current_file_index_bak];
            }
            self.iterations += 1;
            return Some(return_char);
        } else if self.current_file_size_remaining > 0 {
            self.current_file_size_remaining -= 1;
            if self.current_file_size_remaining == 0 {
                self.doing_file = !self.doing_file;
            }
            self.iterations += 1;
            return Some(self.current_file_index - 1);
        }

        return self.next();
    }
}

impl From<String> for DiskMap {
    fn from(value: String) -> Self {
        let input_values: Vec<usize> = value.chars().map(|x| x as usize - 48).collect::<Vec<usize>>();
        let file_lengths: Vec<usize> = input_values.iter().enumerate().filter(|(i, x)| i % 2 == 0).map(|(i, x)| *x).collect::<Vec<usize>>();
        let free_lengths: Vec<usize> = input_values.iter().enumerate().filter(|(i, x)| i % 2 != 0).map(|(i, x)| *x).collect::<Vec<usize>>();
        let mut reversed_lengths: Vec<usize> = file_lengths.clone();
        reversed_lengths.reverse();

        return Self{
            file_lengths: file_lengths.clone(),
            free_lengths: free_lengths,
            file_lengths_bak: reversed_lengths.clone(),
            current_file_index: 0,
            current_free_index: 0,
            current_file_index_bak: 0,
            current_file_size_remaining: 0,
            current_free_size_remaining: 0,
            current_file_size_remaining_bak: reversed_lengths[0].clone(),
            doing_file: true,
            iterations: 0,
            total_sum: file_lengths.clone().iter().sum::<usize>(),
        };
    }
}


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let original: Vec<usize> = input_lines[0].chars().map(|x| x as usize - 48).collect::<Vec<usize>>();
    let mut disk_map: DiskMap = DiskMap::from(input_lines[0].clone());

    println!("{:?}", disk_map);

    let mut final_string: String = String::new();
    let mut final_list: Vec<usize> = Vec::new();
    let mut product_list: Vec<usize> = Vec::new();
    let mut final_sum: usize = 0;
    for (i, x) in disk_map.enumerate() {
        final_string += x.to_string().as_str();
        final_list.push(x);
        product_list.push(x*i);
        final_sum += (x * i);
    }
    // println!("{:?}", final_list);

    // println!("{}", final_string);
    // println!("{:?}", product_list);
    println!("product list sum = {}", product_list.iter().sum::<usize>());
    let input_was_even: bool = original.len() % 2 == 0;
    println!("Input was even {}", input_was_even);


    let all_spaces_sum: usize = original.iter().sum::<usize>();
    let max_index: usize = (original.len() / 2);
    let mut manual_expansion: Vec<usize> = Vec::new();

    let mut file_list: Vec<FileBlock> = Vec::new();

    for i in 0..(max_index + 1) {
        if (2*i) >= original.len() {
            break;
        }
        for j in 0..original[2 * i] {
            manual_expansion.push(i);
        }
        file_list.push(FileBlock { file_index: i, file_size: original[2*i], file_type: FileType::File });

        if (2*i + 1) >= original.len() {
            break;
        }
        for j in 0..original[(2*i) +1] {
            manual_expansion.push(max_index + 1);
        }
        file_list.push(FileBlock { file_index: 0, file_size: original[(2*i) + 1], file_type: FileType::Free });
    }


    if !part_2 {
        // println!("Manual sort = {:?}", manual_expansion);

        let mut reverse_list: Vec<usize> = manual_expansion.iter().filter(|x| **x != (max_index + 1)).copied().collect::<Vec<usize>>();
        let mut manual_expansion_answer: Vec<usize> = Vec::new();

        // println!("Reverse list = {:?}", reverse_list);

        let reverse_list_len: usize = reverse_list.len();

        for i in 0..reverse_list_len {
            if manual_expansion[i] == max_index + 1 {
                if reverse_list.len() == 0 {
                    break;
                }
                manual_expansion_answer.push(reverse_list.pop().expect("List ran out!!!"));
            } else {
                if manual_expansion[i] >= max_index {
                    break;
                }
                manual_expansion_answer.push(manual_expansion[i])
            }
        }

        // println!("Manual sort2 = {:?}", manual_expansion_answer);
        println!("Manual product = {:?}", manual_expansion_answer.iter().enumerate().map(|(i, x)| i * x).sum::<usize>());

        return manual_expansion_answer.iter().enumerate().map(|(i, x)| i * x).sum::<usize>();
    } else {

        // println!("{:?}", file_list);
        let mut i: usize = file_list.len() - 1;
        while i > 0 {
            // println!("{:?}", file_list);
            let current_block = file_list[i].clone();
            // println!("Checking {:?}", current_block);
            if current_block.file_type == FileType::File {
                for j in 0..i {
                    if file_list[j].file_type == FileType::Free && current_block.file_size <= file_list[j].file_size {
                        file_list.remove(i);
                        file_list.insert(i, FileBlock { file_index: 0, file_size: current_block.file_size, file_type: FileType::Free });
                        let empty_block = file_list.remove(j);
                        file_list.insert(j, FileBlock { file_index: current_block.file_index, file_size: current_block.file_size, file_type: FileType::File });
                        if current_block.file_size < empty_block.file_size {
                            file_list.insert(j + 1, FileBlock { file_index: 0, file_size: empty_block.file_size - current_block.file_size, file_type: FileType::Free });
                        }

                        break;
                    }
                }
            }
            i = i - 1;
            // println!("\t{:?}", file_list);
        }

        let mut big_sum: usize = 0;
        i = 0;
        for x in file_list {
            for y in 0..x.file_size {
                big_sum += i * x.file_index;
                i += 1;
            }
        }


        return big_sum;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here
        println!("{:?}", char::from(48));
        println!("{}", "00...111...2...333.44.5555.6666.777.8888".len());
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 60);
    }

    #[test]
    fn example_1_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_2.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 1928);
    }

    #[test]
    fn example_1_3() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_3.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 9786);
    }

    #[test]
    fn example_1_4() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_4.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 2132);
    }

    #[test]
    fn example_1_5() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_5.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 17513);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 6337367222422);

        // 6222777990262 is too low
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_2.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 2858);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 6361380647183);
    }
}
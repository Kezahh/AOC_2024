const INPUTS_FOLDER: &str = "inputs/day_13";

use std::collections::HashMap;

use crate::generic;
use crate::generic::Point;

#[derive(Debug)]
struct Button {
    x: i32,
    y: i32,
    cost: usize,
}

impl Button {
    fn new(value: String, cost: usize) -> Self {
        let x: i32 = value.split(" ").collect::<Vec<_>>()[2][1..].trim_end_matches(",").parse::<i32>().unwrap();
        let y: i32 = value.split(" ").collect::<Vec<_>>()[3][1..].parse::<i32>().unwrap();
        return Self{x: x, y: y, cost: cost};
    }
}

#[derive(Debug)]
struct Machine {
    buttons: Vec<Button>,
    prize: Point,
}

impl Machine {
    fn new(value: &[String]) -> Self {
        // println!("Building Machine from {:?}", value);
        let prize_x: i32 = value[2].split(" ").collect::<Vec<&str>>()[1][2..].trim_end_matches(",").parse::<i32>().unwrap();
        let prize_y: i32 = value[2].split(" ").collect::<Vec<&str>>()[2][2..].parse::<i32>().unwrap();
        return Self { buttons: vec![Button::new(value[0].clone(), 3), Button::new(value[1].clone(), 1)], prize: Point::new(prize_x, prize_y) };
    }

    fn get_cost(&self) -> usize {
        // prize_x = a * button[0].x + b * button[1].x
        // prize_y = a * button[0].y + b * button[1].y
        //
        // a = (prize_x - b * button[1].x) / button[0].x
        //
        // prize_y = ((prize_x - b * button[1].x) / button[0].x) * button[0].y + b * button[1].y
        //
        // prize_y = ((prize_x * button[0].y) / button[0].x) + b * (button[1].y - ((button[1].x * button[0].y) / button[0].x))
        // b = (prize_y - ((prize_x * button[0].y) / button[0].x)) / (button[1].y - ((button[1].x * button[0].y) / button[0].x))

        let b: f32 = (self.prize.y as f32 - ((self.prize.x as f32 * self.buttons[0].y as f32) / self.buttons[0].x as f32))
            / (self.buttons[1].y as f32 - ((self.buttons[1].x as f32 * self.buttons[0].y as f32) / self.buttons[0].x as f32));
        let a: f32 = (self.prize.x as f32 - (b * self.buttons[1].x as f32)) / self.buttons[0].x as f32;

        println!("\ta = {}, b = {}", a, b);
        if a.fract() == 0.0 && b.fract() == 0.0  && a <= 100.0 && b <= 100.0 {
            let cost: usize = (a as usize * self.buttons[0].cost) + (b as usize * self.buttons[1].cost);
            println!("\tCost is {}", cost);
            return cost;
        } else {
            return 0;
        }
    }

    fn get_cost2(&self) -> usize {
        // prize_x = a * button[0].x + b * button[1].x
        // prize_y = a * button[0].y + b * button[1].y
        //
        // a = (prize_x - b * button[1].x) / button[0].x
        //
        // a and b have to be positive integers less than 100;


        return 0;
    }

    fn press_button(&self, current_point: Point, a_presses: usize, b_presses: usize, machine_index: usize, remember: &mut HashMap<usize, HashMap<usize, usize>>) -> usize {
        if !remember.contains_key(&a_presses) {
            remember.insert(a_presses, HashMap::new());
        }

        if a_presses > 100 || b_presses > 100 || current_point.x > self.prize.x || current_point.y > self.prize.y {
            remember.get_mut(&a_presses).unwrap().insert(b_presses, 0);    
            return 0;
        }
        
        if current_point.x == self.prize.x && current_point.y == self.prize.y {
            let cost: usize = (a_presses * self.buttons[0].cost) + (b_presses * self.buttons[1].cost);
            remember.get_mut(&a_presses).unwrap().insert(b_presses, cost);
            return cost;
        }

        if remember.get(&a_presses).unwrap().contains_key(&b_presses) {
            return *remember.get(&a_presses).unwrap().get(&b_presses).unwrap();
        }
        
        // println!("Machine {} :: Running press for {:?} with a press = {} and b_press = {}", machine_index, current_point, a_presses, b_presses);
        let a_result: usize = self.press_button(current_point.walk(self.buttons[0].x, self.buttons[0].y), a_presses + 1, b_presses, machine_index, remember);
        if a_result == 0 {
            let b_result: usize = self.press_button(current_point.walk(self.buttons[1].x, self.buttons[1].y), a_presses, b_presses + 1, machine_index, remember);
            remember.get_mut(&a_presses).unwrap().insert(b_presses, b_result);
            return b_result;
        } else {
            remember.get_mut(&a_presses).unwrap().insert(b_presses, a_result);
            return a_result;
        }
        

    }
}


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut i: usize = 0;
    let mut machines: Vec<Machine> = Vec::new();
    while i < input_lines.len() {
        machines.push(Machine::new(&input_lines[i..(i+3)]));
        i += 4;
    }

    let mut sum_cost: usize = 0;
    let mut recursive_answer: Vec<usize> = Vec::new();
    let mut maths_answer: Vec<usize> = Vec::new();
    for (i, m) in machines.iter().enumerate() {
        println!("{:?}", m);
        let mut remember: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
        maths_answer.push(m.get_cost());
        recursive_answer.push(m.press_button(Point::new(0,0), 0, 0, i, &mut remember));
        sum_cost += recursive_answer.last().unwrap();
    }

    for i in 0..maths_answer.len() {
        if maths_answer[i] != recursive_answer[i] {
            println!("Difference for {}. Maths = {}, recursive = {}", i, maths_answer[i], recursive_answer[i]);
        }
    }

    let mut remember: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    machines[3].press_button(Point::new(0,0), 0, 0, 3, &mut remember);
    machines[3].get_cost();


    return sum_cost;
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
        assert!(answer == 480);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 28753);
        // 16517 too low
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
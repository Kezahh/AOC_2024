const INPUTS_FOLDER: &str = "inputs/day_5";

use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use crate::generic;

#[derive(Debug, PartialEq)]
enum OrderingRuleResult {
    Satisfied,
    NotSatisfied,
    NA,
}

#[derive(Debug)]
struct OrderingRule {
    X: usize,
    Y: usize,
}

impl From<String> for OrderingRule {
    fn from(value: String) -> Self {
        let values: Vec<usize> = value.split("|").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        return Self{X: values[0], Y: values[1]};
    }
}

#[derive(Debug, Clone)]
struct PageProduction {
    pages: Vec<usize>,
}

impl From<String> for PageProduction {
    fn from(value: String) -> Self {
        return Self{pages: value.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>()};
    }
}

impl PageProduction {
    fn get_middle_page(&self) -> usize {
        let mid_index: usize = self.pages.len() / 2;
        return self.pages[mid_index].clone();
    }

    fn check_rule(&self, rule: &OrderingRule) -> OrderingRuleResult {
        match self.pages.iter().position(|x| *x == rule.X) {
            None => OrderingRuleResult::NA,
            Some(rule_page_index) => {
                let start_set: HashSet<usize> = HashSet::from_iter(self.pages[..rule_page_index].iter().cloned());
                if start_set.contains(&rule.Y) {
                    return OrderingRuleResult::NotSatisfied
                } else {
                    return OrderingRuleResult::Satisfied
                }
            }
        }
    }
}

fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let line_break_index: usize = input_lines.iter().position(|x| x == "").unwrap();

    let rules: Vec<OrderingRule> = input_lines[..line_break_index].iter().map(|x| OrderingRule::from(x.clone())).collect::<Vec<OrderingRule>>();
    let page_productions: Vec<PageProduction> = input_lines[(line_break_index + 1)..].iter().map(|x| PageProduction::from(x.clone())).collect::<Vec<PageProduction>>();

    let mut good_productions_sum = 0;
    let mut good_productions_mid_sum = 0;

    let mut bad_productions: Vec<PageProduction> = Vec::new();


    for p in page_productions {
        let mut not_satisfied = false;
        println!("Checking {:?}", p);
        for r in rules.iter() {
            let rule_result: OrderingRuleResult = p.check_rule(r);
            // println!("Rule {:?} = {:?}", r, rule_result);
            if p.check_rule(r) == OrderingRuleResult::NotSatisfied {
                not_satisfied = true;
                bad_productions.push(p.clone());
                break;
            }
        }
        if !not_satisfied {
            good_productions_sum += 1;
            good_productions_mid_sum += p.get_middle_page();
        }
    }

    if part_2 {
        good_productions_mid_sum = 0;
        let mut rules_set: HashMap<usize, Vec<usize>> = HashMap::new();
        for r in rules {
            rules_set.entry(r.Y).or_insert(vec![]).push(r.X);
        }
        println!("{:?}", rules_set);

        for p in bad_productions.iter_mut() {
            println!("\tOld: {:?}", p.pages);
            p.pages.sort_by(|a, b| {
                if rules_set.contains_key(a) {
                    if rules_set.get(a).unwrap().contains(b) {
                        return Ordering::Less;
                    } else {
                        return Ordering::Equal;
                    }
                } else {
                    return Ordering::Equal;
                }
            });
            println!("\tNew: {:?}", p.pages);
            good_productions_mid_sum += p.get_middle_page();
        }
    }
    
    return good_productions_mid_sum;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here
        println!("{:?}", 7.cmp(&8));

        let mut first_list = vec!['a', 'b', 'c', 'g', 'g', 'f', 'd'];
        println!("{:?}", first_list);
        first_list.sort_by(|x, y| {
            println!("\tChecking x = {}, y = {}", x, y);
            if *x == 'f' && *y == 'c' {
                return Ordering::Less;
            } else {
                return x.cmp(y);
            }
        });

        println!("{:?}", first_list);
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 143);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 4959);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 123);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 4655);
    }
}
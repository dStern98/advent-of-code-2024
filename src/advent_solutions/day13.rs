use std::collections::HashMap;

use anyhow::anyhow;

use super::{read_input_file, SolveAdvent};

pub struct Day13;

type OrderedPair = (i64, i64);

#[derive(Debug, Clone)]
struct ClawMachine {
    button_a: OrderedPair,
    button_b: OrderedPair,
    prize_position: OrderedPair, 
}

impl ClawMachine {
    fn try_win_prize_using_algebra(&self) {
        let (mut a_delta_x, mut a_delta_y) = (f64::from(self.button_a.0 as i32), f64::from(self.button_a.1 as i32));
        let (mut b_delta_x, b_delta_y) = (f64::from(self.button_b.0 as i32), f64::from(self.button_b.1 as i32));
        let (mut target_x, mut target_y) = (f64::from(self.prize_position.0 as i32), f64::from(self.prize_position.1 as i32));
        // Equation 1 is 
        a_delta_x *= b_delta_y;
        b_delta_x *= b_delta_y;
        target_x *= b_delta_y;

        a_delta_y *= b_delta_x;
        // b_delta_y *= b_delta_x;
        target_y *= b_delta_x;

        let a_presses = (target_x - target_y) / (a_delta_x - a_delta_y);
        let b_presses = (target_x - a_delta_x * a_presses) / b_delta_x;
        println!("A, B = {},{} presses", a_presses, b_presses);
        
    }
    fn try_win_prize_recursively(&self, current_position: OrderedPair, depth: usize, memo: &mut HashMap<OrderedPair, Option<usize>>) -> Option<usize> {
        if let Some(cache_item) = memo.get(&current_position) {
            return *cache_item
        }
        if depth > 200 || current_position.0 > self.prize_position.0 || current_position.1 > self.prize_position.1 {
            //Return as max depth has been reached
            memo.insert(current_position, None);
            return None;
        }
        if current_position == self.prize_position {
            return Some(0);
        }
        let response_button_a = self.try_win_prize_recursively((current_position.0 + self.button_a.0, current_position.1 + self.button_a.1), depth + 1, memo);
        let response_button_b = self.try_win_prize_recursively((current_position.0 + self.button_b.0, current_position.1 + self.button_b.1), depth + 1, memo);
        if response_button_a.is_none() && response_button_b.is_none() {
            memo.insert(current_position, None);
            return None;
        }
        let response_button_a = response_button_a.unwrap_or(usize::MAX - 3) + 3; // It costs 3 tokens press button A
        let response_button_b = response_button_b.unwrap_or(usize::MAX - 1) + 1; // It costs 1 token to press button b
        let result = Some(response_button_a.min(response_button_b));
        memo.insert(current_position, result);
        result
        
    }
    fn parse_prize_into_ordered_pair(line: &str) -> anyhow::Result<OrderedPair> {
        let trimmed_num = line.replace("Prize:", "").replace("X=", "").replace("Y=", "");
        let isolated_nums: Result<Vec<_>, _> = trimmed_num.split(',').filter_map(|item| {
            if !item.is_empty() {
                return Some(item.trim())
            }
            None
        }).map(|item| item.parse::<i64>()).collect();
        let [x_plus, y_plus]: [i64; 2] = isolated_nums?.try_into().map_err(|_| anyhow!("Could not coerce into length 2 array"))?;
        Ok((x_plus, y_plus))
    }
    fn parse_button_into_ordered_pair(line: &str) -> anyhow::Result<OrderedPair> {
        let trimmed_num = line.trim().replace("Button A:", "").replace("Button B:", "").replace("X+", "").replace("Y+", "");
        let isolated_nums: Result<Vec<_>, _> = trimmed_num.split(',').filter_map(|item| {
            if !item.is_empty() {
                return Some(item.trim())
            }
            None
        }).map(|item| item.parse::<i64>()).collect();
        let [x_plus, y_plus]: [i64; 2] = isolated_nums?.try_into().map_err(|_| anyhow!("Could not coerce into length 2 array"))?;
        Ok((x_plus, y_plus))
    
    }
    fn construct_claws(file_input: &str) -> anyhow::Result<Vec<ClawMachine>>{
        let mut button_a_content = None;
        let mut button_b_content = None;
        let mut prize = None;
        let mut constructed_claw_machines = Vec::new();
        for line in file_input.lines() {
            if line.starts_with("Button A") {
                button_a_content = Some(ClawMachine::parse_button_into_ordered_pair(line)?);
            }
            else if line.starts_with("Button B") {
                button_b_content = Some(ClawMachine::parse_button_into_ordered_pair(line)?);
            }else if line.starts_with("Prize:") {
                prize = Some(ClawMachine::parse_prize_into_ordered_pair(line)?)
            } else {
                //We flush here
                constructed_claw_machines.push(ClawMachine {
                    button_a: button_a_content.ok_or(anyhow!("Button A was None when it should be set!"))?,
                    button_b: button_b_content.ok_or(anyhow!("Button B was None when it should be set!"))?,
                    prize_position: prize.ok_or(anyhow!("Prize was None when it should be set!"))?,
                });
                button_a_content = None;
                button_b_content = None;
                prize = None;
            }
        }
        constructed_claw_machines.push(ClawMachine {
            button_a: button_a_content.ok_or(anyhow!("Button A was None when it should be set!"))?,
            button_b: button_b_content.ok_or(anyhow!("Button B was None when it should be set!"))?,
            prize_position: prize.ok_or(anyhow!("Prize was None when it should be set!"))?,
        });
        Ok(constructed_claw_machines)
    }
}



impl SolveAdvent for Day13 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let claw_machines = ClawMachine::construct_claws(&file_contents)?;
        let mut min_total_tokens = 0;
        for claw_machine in claw_machines {
            let mut memo = HashMap::new();
            if let Some(minimum_tokens_to_win) = claw_machine.try_win_prize_recursively((0, 0), 0, &mut memo) {
                min_total_tokens += minimum_tokens_to_win;
            }

        }
        println!("Minimum total tokens to win is {}", min_total_tokens);
        Ok(())
    }

    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
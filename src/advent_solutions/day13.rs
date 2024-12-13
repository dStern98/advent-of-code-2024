
use anyhow::anyhow;

use super::{read_input_file, SolveAdvent};

pub struct Day13;

type OrderedPair = (f64, f64);

#[derive(Debug, Clone)]
struct ClawMachine {
    button_a: OrderedPair,
    button_b: OrderedPair,
    prize_position: OrderedPair, 
}

impl ClawMachine {
    fn try_win_prize_using_algebra(&self) -> Option<i64> {
        //!Use Algebra to solve the problem
        //! It turns out there the is exactly one solution to any given Claw Machine 
        //! problem, so there is actually nothing to optimize!
        //! Suppose that we are using example 3 from the problem: 
        //! 
        //! Button A: X+17, Y+86
        //! Button B: X+84, Y+37
        //! Prize: X=7870, Y=6450
        let (a_delta_x, a_delta_y) = (self.button_a.0, self.button_a.1);
        let (b_delta_x, b_delta_y) = (self.button_b.0, self.button_b.1);
        let (target_x, target_y) = (self.prize_position.0, self.prize_position.1);
        // Equation 1 is The x's equation: 17A + 84B = 7870
        let scaled_a_dx = a_delta_x * b_delta_y;
        let scaled_target_x = target_x * b_delta_y;
        //Equation 2 is the Y's equation: 86A + 37B = 6450
        let scaled_a_dy = a_delta_y * b_delta_x;
        let scaled_target_y =   target_y * b_delta_x;

        let a_presses = (scaled_target_x - scaled_target_y) / (scaled_a_dx - scaled_a_dy);
        let b_presses = (target_x - (a_presses * a_delta_x)) / b_delta_x;
        if !a_presses.is_finite() || !b_presses.is_finite() || a_presses.is_sign_negative() || b_presses.is_sign_negative() {
            return None;
        }

        if a_presses % 1.0 == 0.0 && b_presses % 1.0 == 0.0 {
            let a_presses = a_presses.trunc() as i64;
            let b_presses = b_presses.trunc() as i64;
            return Some(a_presses * 3 + b_presses);
        }
        None
        
    }

    fn parse_prize_into_ordered_pair(line: &str) -> anyhow::Result<OrderedPair> {
        let trimmed_num = line.replace("Prize:", "").replace("X=", "").replace("Y=", "");
        let isolated_nums: Result<Vec<_>, _> = trimmed_num.split(',').filter_map(|item| {
            if !item.is_empty() {
                return Some(item.trim())
            }
            None
        }).map(|item| item.parse::<f64>()).collect();
        let [x_plus, y_plus]: [f64; 2] = isolated_nums?.try_into().map_err(|_| anyhow!("Could not coerce into length 2 array"))?;
        Ok((x_plus, y_plus))
    }
    fn parse_button_into_ordered_pair(line: &str) -> anyhow::Result<OrderedPair> {
        let trimmed_num = line.trim().replace("Button A:", "").replace("Button B:", "").replace("X+", "").replace("Y+", "");
        let isolated_nums: Result<Vec<_>, _> = trimmed_num.split(',').filter_map(|item| {
            if !item.is_empty() {
                return Some(item.trim())
            }
            None
        }).map(|item| item.parse::<f64>()).collect();
        let [x_plus, y_plus]: [f64; 2] = isolated_nums?.try_into().map_err(|_| anyhow!("Could not coerce into length 2 array"))?;
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
        let mut total_tokens_used = 0;
        for claw_machine in claw_machines {
            if let Some(solution) = claw_machine.try_win_prize_using_algebra() {
                total_tokens_used += solution;
            }

        }
        println!("Total tokens used is {}", total_tokens_used);
        Ok(())
    }

    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        //! In theory the code here should work, but most likely a floating point error is causing issues
        let file_contents = read_input_file(path_to_file)?;
        let mut claw_machines = ClawMachine::construct_claws(&file_contents)?;
        for claw_machine in claw_machines.iter_mut() {
            claw_machine.prize_position.0 += 10000000000000.0;
            claw_machine.prize_position.1 += 10000000000000.0;
        }
        let mut total_tokens_used = 0;
        for claw_machine in claw_machines {
            if let Some(solution) = claw_machine.try_win_prize_using_algebra() {
                total_tokens_used += solution;
            }

        }
        println!("Total tokens used is {}", total_tokens_used);
        Ok(())
    }
}
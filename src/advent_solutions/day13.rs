
use anyhow::anyhow;

use super::{read_input_file, SolveAdvent};

pub struct Day13;

type OrderedPair = (f64, f64);

///A single Claw Machine.
/// Every click of `button_a` will move the position
/// of the claw by x+button_a_dx, y+button_a_dy 
#[derive(Debug, Clone)]
struct ClawMachine {
    ///The delta_x, delta_y moved by the claw whenever
    /// button_a is pressed
    button_a: OrderedPair,
    ///The delta_x, delta_y moved by the claw whenever 
    /// button_b is pressed
    button_b: OrderedPair,
    ///The ordered pair position of the target prize
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
        //! Equation 1 is The x's equation: 17A + 84B = 7870
        //!Equation 2 is the Y's equation: 86A + 37B = 6450
        //! Rudimentary substitution/elimination from Algebra 1 will solve for the unique A, B 
        //! pair that completes the equation.

        //Use Algebra 1 elimination to isolate the A's to solve for A
        let (a_delta_x, a_delta_y) = (self.button_a.0, self.button_a.1);
        let (b_delta_x, b_delta_y) = (self.button_b.0, self.button_b.1);
        let (target_x, target_y) = (self.prize_position.0, self.prize_position.1);
        let scaled_a_dx = a_delta_x * b_delta_y;
        let scaled_target_x = target_x * b_delta_y;
        let scaled_a_dy = a_delta_y * b_delta_x;
        let scaled_target_y =   target_y * b_delta_x;

        let a_presses = (scaled_target_x - scaled_target_y) / (scaled_a_dx - scaled_a_dy);
        //Once we solve for A, we can trivially solve for B
        let b_presses = (target_x - (a_presses * a_delta_x)) / b_delta_x;
        if !a_presses.is_finite() || !b_presses.is_finite() || a_presses.is_sign_negative() || b_presses.is_sign_negative() {
            return None;
        }
        if a_presses % 1.0 < 0.000001 || b_presses % 1.0 < 0.000001 {
            let a_presses = a_presses.trunc() as i64;
            let b_presses = b_presses.trunc() as i64;
            return Some(a_presses * 3 + b_presses);
        }
        None
        
    }

    fn parse_prize_line_into_ordered_pair(line: &str) -> anyhow::Result<OrderedPair> {
        //! Parse lines that look like this `Prize: X=8400, Y=5400` into `8400.0, 5400.0`
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
    fn parse_button_line_into_ordered_pair(line: &str) -> anyhow::Result<OrderedPair> {
        //! Parse lines that look like this: `Button A: X+94, Y+34` into `94.0, 34.0`
        //! The chain of `replace` calls is inefficient but I do not need to use Regex to optimize
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
        //! Parse the file input into claw machines.
        let mut button_a_content = None;
        let mut button_b_content = None;
        let mut prize = None;
        let mut constructed_claw_machines = Vec::new();
        for line in file_input.lines() {
            if line.starts_with("Button A") {
                button_a_content = Some(ClawMachine::parse_button_line_into_ordered_pair(line)?);
            }
            else if line.starts_with("Button B") {
                button_b_content = Some(ClawMachine::parse_button_line_into_ordered_pair(line)?);
            }else if line.starts_with("Prize:") {
                prize = Some(ClawMachine::parse_prize_line_into_ordered_pair(line)?)
            } else {
                //We flush the Options whenever the we reach a blank line
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
        //Flush one more time because the last machine has no trailing empty line
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
        //! Despite the framing of the problem, this is not actually an optimization problem!!!
        //! There are two unknowns and two linear equations for each claw machine. Which means there is always
        //! either no solution or exactly 1 solution (number of A, B presses) that reached the target. Minimizing the tokens
        //! is a red herring, as there is nothing to minimize (there being only 1 solution).
        //! The only question is whether the solution (when it exists) are two whole numbers or not.
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
        //! Exactly the same solution as part1, but add the massive number to each
        //! prize position before computing. 
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
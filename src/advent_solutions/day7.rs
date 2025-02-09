use anyhow::{anyhow, Context};

use super::{read_input_file, SolveAdvent};


pub struct Day7;

///A single equation line. 
/// It is not guranteed that the `target` value
/// can be constructed from the `constituents`
#[derive(Debug, Clone)]
struct Equation {
    target: i64,
    constituents: Vec<i64>
}

///An operation to combine two numbers.
enum AvailableOperators {
    Plus, 
    Multiply, 
    Concatenate
}

impl AvailableOperators {
    fn apply(&self, num1: i64, num2: i64) -> i64 {
        //! Apply the specified operation to the two numbers.
        match self {
            AvailableOperators::Plus => num1 + num2,
            AvailableOperators::Multiply => num1 * num2,
            AvailableOperators::Concatenate => {
                let combined_str = num1.to_string() + &num2.to_string();
                combined_str.parse::<i64>().expect("Concatenation op can never fail!")
            }
        }
    }
}



impl Equation {
    fn try_new(line: &str) -> anyhow::Result<Equation> {
        let [target, constituents]: [&str;2] = line.trim().split(':').collect::<Vec<_>>().try_into().map_err(|_err| anyhow!("Failed to coerce into length 2 array"))?;
        let target = target.parse::<i64>().context("Target is not a valid integer")?;
        let constituents: Result<Vec<_>, _> = constituents.trim().split(' ').map(|constituent| constituent.parse::<i64>()).collect();
        let constituents = constituents?;
        Ok(Equation {
            target, 
            constituents
        })
    }


    fn is_possible(&self, available_operations: &[AvailableOperators]) -> bool {
        //! Determine if the `target` sum can be generated by some combination of the `constituents`
        //! using the `available_operations`.
        let mut scratchpad = Vec::new();
        let mut idx = 0;
        while idx < self.constituents.len() {
            if scratchpad.is_empty() {
                scratchpad.push(self.constituents[idx]);
            } else {
                let previous_scratchpad = std::mem::take(&mut scratchpad);
                for previous_value in previous_scratchpad {
                    for available_operator in available_operations {
                        scratchpad.push(available_operator.apply(previous_value, self.constituents[idx]));
                    }
                }
            }
            idx += 1;
        }
        scratchpad.contains(&self.target)
    }
}

impl SolveAdvent for Day7 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let equations: Result<Vec<_>, _> = file_contents.lines().map(Equation::try_new).collect();
        let equations = equations.context("Could not parse all lines into equations")?;
        let mut test_value_sum = 0;
        let available_operations = [AvailableOperators::Plus, AvailableOperators::Multiply];
        for equation in equations.iter() {
            if equation.is_possible(&available_operations) {
                test_value_sum += equation.target;
            }
        }
        println!("Total of all possible equations for part1 is {:?}", test_value_sum);
        Ok(())
    }

    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        //! Exactly the same code as part1 solution but the `available_operations` array
        //! contains the new concatenation operation (`||`) as well as the `+`/`*` ops.
        let file_contents = read_input_file(path_to_file)?;
        let equations: Result<Vec<_>, _> = file_contents.lines().map( Equation::try_new).collect();
        let equations = equations.context("Could not parse all lines into equations")?;
        let mut test_value_sum = 0;
        let available_operations = [AvailableOperators::Plus, AvailableOperators::Multiply, AvailableOperators::Concatenate];
        for equation in equations.iter() {
            if equation.is_possible(&available_operations) {
                test_value_sum += equation.target;
            }
        }
        println!("Total of all possible equations for part2 is {:?}", test_value_sum);
        Ok(())
    }
}
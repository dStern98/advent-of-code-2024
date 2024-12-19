
use anyhow::{anyhow, Context};

use super::{read_input_file, SolveAdvent};

pub struct Day17;

///Stores the computer runtime
#[derive(Debug, Clone)]
struct ChronospatialComputer {
    ///Register A value
    ra: i64,
    ///Register B value
    rb: i64,
    ///Register C Value
    rc: i64, 
    ///Current Instruction Pointer
    instruction_ptr: usize, 
    ///Output buffer for the `out` command
    out_buffer: Vec<u8>
}

fn parse_input_file(file_contents: &str) -> anyhow::Result<(ChronospatialComputer, Vec<u8>)> {
    let register_program_divide = file_contents.find("\n\r").ok_or(anyhow!("Input file did not contain a program/computer init demarcation point"))?;
    let register_configs = &file_contents[0..register_program_divide];
    let [ra, rb, rc]: [&str; 3] = register_configs.trim().split("\r\n").collect::<Vec<_>>().try_into().map_err(|_| anyhow!("Failed to coerce register config to exactly 3 items"))?;
    let ra = ra.replace("Register A:", "").trim().parse::<i64>()?;
    let rb = rb.replace("Register B:", "").trim().parse::<i64>()?;
    let rc = rc.replace("Register C:", "").trim().parse::<i64>()?;
    let computer = ChronospatialComputer::new(ra, rb, rc);
    let program = &file_contents[register_program_divide..];
    let program_instructions = program.trim().replace("Program:", "").trim().split(',').map(|num| num.parse::<u8>()).collect::<Result<Vec<_>, _>>()?;
    Ok((computer, program_instructions))
}

impl ChronospatialComputer {
    fn new(ra: i64, rb: i64, rc: i64) -> Self {
        ChronospatialComputer {
            ra,
            rb,
            rc,
            instruction_ptr: 0,
            out_buffer: Vec::new()
        }
    }
    fn resolve_combo_operand(&self, operand: u8) -> anyhow::Result<i64> {
        //! Resolve the combo `operand` to a value
        match operand {
            0..=3 => Ok(operand  as i64),
            4 => Ok(self.ra),
            5 => Ok(self.rb),
            6 => Ok(self.rc),
            7 => anyhow::bail!("Encountered illegal operand 7"),
            other => anyhow::bail!("Encountered illegal operand {}", other),
        }
    }
     fn execute_instruction(&mut self, opcode: u8, operand: u8) -> anyhow::Result<()> {
        //! Match and execute the opcode
        match opcode {
            0 => self.adv(operand).context(format!("Error executing adv command with opcode {} and operand {}", opcode, operand))?,
            1 => self.bxl(operand).context(format!("Error executing bxl command with opcode {} and operand {}", opcode, operand))?,
            2 => self.bst(operand).context(format!("Error executing bst command with opcode {} and operand {}", opcode, operand))?,
            3 => self.jnz(operand).context(format!("Error executing jnz command with opcode {} and operand {}", opcode, operand))?,
            4 => self.bxc(operand).context(format!("Error executing bxc command with opcode {} and operand {}", opcode, operand))?,
            5 => self.out(operand).context(format!("Error executing out command with opcode {} and operand {}", opcode, operand))?,
            6 => self.bdv(operand).context(format!("Error executing bdv command with opcode {} and operand {}", opcode, operand))?,
            7 => self.cdv(operand).context(format!("Error executing cdv command with opcode {} and operand {}", opcode, operand))?,
            other => anyhow::bail!("Encountered illegal opcode {}", other)
        };
        Ok(())
    }

    fn run(&mut self, program: &[u8]) -> anyhow::Result<()> {
        //! Run the program until the instruction pointer goes off the map
        while let Some(instructions) = program.get(self.instruction_ptr..self.instruction_ptr + 2) {
            let opcode = instructions[0];
            let operand = instructions[1];
            self.execute_instruction(opcode, operand)?;
        }
        Ok(())
    }

    fn adv(&mut self, operand: u8) -> anyhow::Result<()> {
        let combo_operand = self.resolve_combo_operand(operand)?;
        let result = (self.ra as f64) / 2.0_f64.powi(combo_operand as i32);
        self.ra = result.trunc() as i64;
        self.instruction_ptr += 2;
        Ok(())
    }

    fn bxl(&mut self, operand: u8) -> anyhow::Result<()> {
        self.rb ^= operand as i64;
        self.instruction_ptr += 2;
        Ok(())
    }

    fn bst(&mut self, operand: u8) -> anyhow::Result<()> {
        let combo_operand = self.resolve_combo_operand(operand)?;
        self.rb = combo_operand % 8;
        self.instruction_ptr += 2;
        Ok(())
    }

    fn jnz(&mut self, operand: u8) -> anyhow::Result<()> {
        //! Jump if register a value is not 0
        if self.ra != 0 {
            self.instruction_ptr = operand as usize;
        } else {
            self.instruction_ptr += 2; //Only increment instruction pointer by 2 if no jump occurred
        }
        Ok(())
    }

    fn bxc(&mut self, _operand: u8) -> anyhow::Result<()> {
        self.rb ^= self.rc;
        self.instruction_ptr += 2;
        Ok(())
    }

    fn out(&mut self, operand: u8) -> anyhow::Result<()> {
        let combo_operand = self.resolve_combo_operand(operand)? % 8;
        self.out_buffer.push(combo_operand as u8);
        self.instruction_ptr += 2;
        Ok(())
    }

    fn bdv(&mut self, opcode: u8) -> anyhow::Result<()> {
        let combo_operand = self.resolve_combo_operand(opcode)?;
        let result = (self.ra as f64) / 2.0_f64.powi(combo_operand as i32);
        self.rb = result.trunc() as i64;
        self.instruction_ptr += 2;
        Ok(())
    }

    fn cdv(&mut self, opcode: u8) -> anyhow::Result<()> {
        let combo_operand = self.resolve_combo_operand(opcode)?;
        let result = (self.ra as f64) / 2.0_f64.powi(combo_operand as i32);
        self.rc = result.trunc() as i64;
        self.instruction_ptr += 2;
        Ok(())
    }
}

impl SolveAdvent for Day17 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let (mut computer, program_instructions) = parse_input_file(&file_contents)?;
        computer.run(&program_instructions)?;
        println!("Out contains the following: {:?}", computer.out_buffer.iter().map(|num| num.to_string()).collect::<Vec<_>>().join(","));
        Ok(())
    }

    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        //! Brute force solution will not resolve in a reasonable amount of time
        let file_contents = read_input_file(path_to_file)?;
        let (computer, program_instructions) = parse_input_file(&file_contents)?;
        for register_a_init in 1..=i64::MAX {
            let mut computer = computer.clone();
            computer.ra = register_a_init;
            computer.run(&program_instructions)?;
            if computer.out_buffer == program_instructions {
                println!("Register a init value {} generates out buffer copy of input program", register_a_init);
                break;
            }
        }
        Ok(())
    }
}
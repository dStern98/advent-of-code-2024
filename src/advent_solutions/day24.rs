use std::collections::{HashMap, VecDeque};

use anyhow::{anyhow, Context};

use super::{read_input_file, SolveAdvent};

pub struct Day24;

#[derive(Debug, Clone, Copy)]
enum GateType{
    And,
    Or,
    Xor
}

impl GateType {
    fn eval(&self, in_1: bool, in_2: bool) -> bool{
        //!Execute the operation specified by the gate type
        match self {
            GateType::And => in_1 && in_2,
            GateType::Or => in_1 || in_2,
            GateType::Xor => in_1 ^ in_2 
        }
    }
}

///Represents a single gate junction
/// It is not guranteed that we can evaluate the gate at
/// any given time.
#[derive(Debug, Clone)]
struct GateJunction {
    ///The variable name of input_1, such as `x01`
    in_1: String,
    ///The variable name of input_2, such as `y02`
    in_2: String,
    ///The type of gate this junction is
    gate_type: GateType,
    ///The variable name of the ouput, such as `z00`
    out: String
}

///The ValueStore stores a mapping of each variable whose
/// associated value has been computed, or given in the problem input.
type ValueStore = HashMap<String, bool>;

fn init_value_store(input: &str) -> anyhow::Result<ValueStore> {
    //! Construct the value store
    let mut value_store = HashMap::new();
    for line in input.trim().lines() {
        let [input_name, input_value]: [&str; 2] = line.split(':').collect::<Vec<_>>().try_into().map_err(|_| anyhow!("Could not split init line correctly"))?;
        let init_value = match input_value.trim() {
            "0" => false,
            "1" => true,
            other => anyhow::bail!("Encountered invalid initialization value {} not a 0 or 1", other)
        };
        value_store.insert(input_name.to_string(), init_value);
    }
    Ok(value_store)
}

fn generate_final_number(value_store: ValueStore) -> anyhow::Result<u64> {
    //! Parse all of the numbers starting with `z` to produce a final 64-bit number
    let mut parsed_z_values = Vec::new();
    for (gate, value) in value_store.iter() {
        if gate.starts_with("z") {
            let temp_key = gate.replace("z0", "").replace("z", "").parse::<u8>()?;
            parsed_z_values.push((temp_key, *value as u8));
        }
    }
    parsed_z_values.sort_by_key(|(gate_num, _value)| *gate_num);
    let binary_number  = parsed_z_values.into_iter().rev().map(|(_gate, value)| value.to_string()).collect::<String>();
    let final_number = u64::from_str_radix(&binary_number, 2).context(format!("Invalid binary number {}", binary_number))?;
    Ok(final_number)
}

impl GateJunction{
    fn from_line(line: &str) -> anyhow::Result<Self> {
        let [ins, out]: [&str; 2]  = line.split("->").collect::<Vec<_>>().try_into().map_err(|_| anyhow::anyhow!("Could not construct gate junction from input {line}"))?;
        let [in1, gate_type, in2]: [&str; 3] = ins.trim().split(' ').collect::<Vec<_>>().try_into().map_err(|_| anyhow!("Failed to coerce gate input"))?;
        let gate_type = match gate_type {
            "AND" => GateType::And,
            "XOR" => GateType::Xor,
            "OR" => GateType::Or,
            other => anyhow::bail!("Encountered invalid gate type {}", other)
        };
        Ok(GateJunction {
            in_1: in1.trim().to_string(),
            in_2: in2.trim().to_string(),
            gate_type,
            out: out.trim().to_string()
        })
    }

    fn try_eval(&self, value_store: &ValueStore) -> Option<(String, bool)> {
        //! Attempt to evaluate the value of the gate junction. It is possible that at
        //! the current time we do not know both input values, in which case no progress can be made
        if let Some(value1) = value_store.get(&self.in_1) {
            if let Some(value2) = value_store.get(&self.in_2) {
                let result = self.gate_type.eval(*value1, *value2);
                return Some((self.out.clone(), result));
            }
        }
        None
    }
}

fn parse_input_file(file_contents: &str) -> anyhow::Result<(Vec<GateJunction>, ValueStore)> {
    let split_position = file_contents.find("\r\n\r\n").ok_or(anyhow!("Input file did not contain an empty line as expected"))?;
    let value_store = init_value_store(&file_contents[0..split_position])?;
    let gate_junctions = (file_contents[split_position..]).trim().lines().map(GateJunction::from_line).collect::<Result<Vec<_>, _>>()?;
    Ok((gate_junctions, value_store))
}

impl SolveAdvent for Day24{
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let (gate_junctions, mut value_store) = parse_input_file(&file_contents)?;
        let mut gate_junctions = gate_junctions.into_iter().collect::<VecDeque<_>>();
        //Iterate over gate junctions, evaluating each gate where possible. If a gate is successfully evaluated,
        //its output value is written in the `value_store` and the `gate_junction` is removed from the `gate_junctions` queue
        while let Some(current_junction) = gate_junctions.pop_front() {
            match current_junction.try_eval(&value_store) {
                Some((out_name, out_val)) => {
                    value_store.insert(out_name, out_val);
                },
                None => {
                    //If we cannot at present evaluate the gate junction, put it in the back of the queue.
                    //Try again at a later time when other gates have been evaluated.
                    gate_junctions.push_back(current_junction);
                }
            }
        }
        let final_number = generate_final_number(value_store)?;
        println!("Final number produced is: {}", final_number);
        Ok(())
    }

    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
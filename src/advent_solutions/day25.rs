use anyhow::anyhow;

use super::{read_input_file, SolveAdvent};

pub struct Day25;

#[derive(Debug, Clone, Copy)]
enum KeyOrLock {
    Lock,
    Key
}

#[derive(Debug, Clone, Copy)]
struct Schematic {
    schematic_type: KeyOrLock,
    //The heights of each column in the key or lock
    heights: [u64;5]
}

impl Schematic {
    fn from_schematic_diagram(schematic: &str) -> anyhow::Result<Self> {
        let schematic = schematic.trim().lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let obj_type = {
            if schematic[0].iter().all(|symbol| symbol == &'.') {
                //Keys have an empty first row
                KeyOrLock::Key
            } else if schematic.last().unwrap().iter().all(|symbol| symbol == &'.') {
                //locks have an empty last row
                KeyOrLock::Lock
            } else {
                anyhow::bail!(format!("Schematic {:?} could not be classified as a lock or key", schematic));
            }
        };
        let mut heights = Vec::with_capacity(5);
        for col_number in 0..schematic[0].len() {
            let mut height = 0;
            for row in schematic.iter() {
                if row[col_number] == '#' {
                    height += 1;
                }
            }
            //Check before subtracting one to prevent wrapping the u64 in case its 0, which should never happen
            anyhow::ensure!(height>= 1, anyhow!("Encountered a column whose height is 0, which is illegal!"));
            heights.push(height - 1); // the rules of the problem do not count one of the rows as a height so subtract 1
        }
        let heights: [u64; 5] = heights.try_into().map_err(|_| anyhow!("Collected heights were not of length 5"))?;
        Ok(Schematic {
            schematic_type: obj_type,
            heights
        })
    }
}

fn parse_schematics(file_contents: &str) -> anyhow::Result<Vec<Schematic>> {
    let parsed_schematics = file_contents.split("\r\n\r\n").map(Schematic::from_schematic_diagram).collect::<Result<Vec<_>, _>>()?;
    Ok(parsed_schematics)
}

fn is_valid_lock_key_pair(lock: &[u64; 5], key: &[u64; 5]) -> bool {
    //! Check if a lock/key pair is valid
    for (lock_val, key_val) in lock.iter().zip(key.iter()) {
        if *lock_val + *key_val > 5 {
            return false
        }
    }
    true
}

impl SolveAdvent for Day25 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let parsed_schematics = parse_schematics(&file_contents)?;
        //Seperate locks and keys. It would probably be more idiomatic to have two distinct
        //types so that the `is_valid_lock_key_pair` could be protected using types from bugs.
        let mut locks = Vec::new();
        let mut keys = Vec::new();
        for schematic in parsed_schematics {
            match schematic.schematic_type {
                KeyOrLock::Key => keys.push(schematic.heights),
                KeyOrLock::Lock => locks.push(schematic.heights),
            }
        }

        let mut valid_lock_key_pairs = 0;
        for key in keys.iter() {
            for lock in locks.iter() {
                if is_valid_lock_key_pair(lock, key) {
                    valid_lock_key_pairs += 1;
                }
            }
        }
        println!("There are a total of {} unique lock-key pairs that are valid", valid_lock_key_pairs);
        Ok(())
    }

    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
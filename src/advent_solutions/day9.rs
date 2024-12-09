use anyhow::anyhow;

use super::{read_input_file, SolveAdvent};

pub struct Day9;

#[derive(Debug, Clone)]
enum MemoryBlock {
    Free,
    Occupied{file_id: usize}
}

impl MemoryBlock {
    fn construct_memory_blocks(file_contents: &str) -> anyhow::Result<Vec<MemoryBlock>> {
        let mut memory_blocks = Vec::new();
        let mut latest_file_number = 0;
        for (position, space_size) in file_contents.chars().enumerate()
        {
            let space_size = space_size.to_digit(10).ok_or(anyhow!("Could not convert available space to a number"))?;
            if position % 2 == 0 {
                for _ in 0..space_size {
                    memory_blocks.push(MemoryBlock::Occupied { file_id: latest_file_number});
                }
                latest_file_number += 1;
            }
            else {
                for _ in 0..space_size {
                    memory_blocks.push(MemoryBlock::Free);
                }
            }
        }
        Ok(memory_blocks)
    }

    fn compactify_memory_blocks(memory_blocks: &mut [MemoryBlock]) -> Option<()> {
        let mut lower_bound = 0;
        let mut upper_bound = memory_blocks.len() - 1;
        loop {
            let next_available_space  = seek_next_free_memory_block(memory_blocks, lower_bound)?;
            let next_occupied_space = seek_next_occupied_memory_block(memory_blocks, upper_bound)?;
            if next_available_space > next_occupied_space {
                return Some(());
            }
            memory_blocks.swap(next_available_space, next_occupied_space);
            lower_bound = next_available_space;
            upper_bound = next_occupied_space;
        }
    }
}

fn seek_next_free_memory_block(memory_blocks: &[MemoryBlock], lower_bound: usize) -> Option<usize> {
    let mut seek_position = lower_bound;
    while seek_position < memory_blocks.len() {
        if let MemoryBlock::Free = memory_blocks.get(seek_position).unwrap() {
            return Some(seek_position);
        }
        seek_position += 1;
    }
    None
}

fn seek_next_occupied_memory_block(memory_blocks: &[MemoryBlock], upper_bound: usize) -> Option<usize> {
    let mut seek_position = upper_bound as i64;
    while seek_position >= 0 {
        if let MemoryBlock::Occupied { .. } = memory_blocks.get(seek_position as usize).unwrap() {
            return Some(seek_position as usize)
        }
        seek_position -=1;
    }
    None
}

impl SolveAdvent for Day9 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let mut memory_blocks = MemoryBlock::construct_memory_blocks(&file_contents)?;
        MemoryBlock::compactify_memory_blocks(&mut memory_blocks);
        let mut checksum = 0;
        for (position, memory_block) in memory_blocks.iter().enumerate() {
            if let MemoryBlock::Occupied { file_id } = memory_block {
                checksum += position * (*file_id);
            }
        } 
        println!("Final checksum after memory compaction is {}", checksum);
        Ok(())
    }

    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
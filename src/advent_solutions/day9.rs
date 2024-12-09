use anyhow::anyhow;

use super::{read_input_file, SolveAdvent};

pub struct Day9;

///Represents the state of a single block of memory, 
/// which can either be free or occupied by file with id `file_id`.
#[derive(Debug, Clone)]
enum MemoryBlock {
    Free,
    Occupied{file_id: usize}
}

///Represents the full span of a file, which starts
/// at `start_index` and ends at `end_index` inclusive. 
/// `file_id` is used mostly for debugging.
#[derive(Debug, Clone)]
struct FileBlock {
    start_index: usize,
    end_index: usize,
    #[allow(dead_code)]
    file_id: usize
}

impl FileBlock {
    fn construct_file_block(memory_blocks: &[MemoryBlock], sentinal_file_id: usize, lower_idx: usize) -> FileBlock {
        //! Build a `FileBlock`. It is the callers job to gurantee that the block at `lower_idx` is free, as this is assumed
        //! rather than checked. The `sentinal_file_id` is the `file_id` of the block. This sentinal gurantees that two adjacent
        //! files will not accidentally be put into one continous block.
        let mut upper_idx = lower_idx + 1;
        while let Some(MemoryBlock::Occupied { file_id }) = memory_blocks.get(upper_idx){ 
                if file_id == &sentinal_file_id {
                    upper_idx += 1;
                    continue;
                } else {
                    break;
                }
        }
        FileBlock {
            start_index: lower_idx,
            end_index: upper_idx -1, //upper_idx is always one greather than desired
            file_id: sentinal_file_id
        }
    }
    fn from_memory_block(memory_blocks: &[MemoryBlock]) -> Vec<FileBlock> {
        //! Parse the `memory_blocks` slice into a list of all files.
        let mut lower_idx = 0;
        let mut file_blocks = Vec::new();
        while lower_idx < memory_blocks.len() {
            if let Some(MemoryBlock::Occupied { file_id }) = memory_blocks.get(lower_idx) {
                let file_block = FileBlock::construct_file_block(memory_blocks, *file_id, lower_idx);
                lower_idx = file_block.end_index + 1;
                file_blocks.push(file_block); 
            } else {
                lower_idx += 1;
            }
        }
        file_blocks
    }
}

impl MemoryBlock {
    fn construct_memory_blocks(file_contents: &str) -> anyhow::Result<Vec<MemoryBlock>> {
        //! Construct memory blocks as described in the problem.
        //! 
        //! Input `2333133121414131402` -> `00...111...2...333.44.5555.6666.777.888899`,
        //! where `MemoryBlock::Free` is the `.` and the numbers represents the file_id of an occupied memory block.
        //! The input numbers alternate between number of free spaces and number of spaces occupied by a file block.
        let mut memory_blocks = Vec::new();
        let mut latest_file_number = 0;
        for (position, space_size) in file_contents.chars().enumerate()
        {
            //The space_size number represents how many consecutive memory blocks are either free or occupied by the file
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

    fn compactify_memory_blocks_without_fragmentation(memory_blocks: &mut [MemoryBlock]) {
        //! Without fragmentation, we can only swap memory if the entire file block can fit into the memory slot.
        let mut file_blocks = FileBlock::from_memory_block(memory_blocks);
        //The `file_blocks` vec is already sorted in ascending order of `file_id`, so taking from the back will satisfy the requirement 
        // to: "attempt to move each file exactly once in order of decreasing file ID number starting with the file with the highest file ID number"
        while let Some(file_block) = file_blocks.pop() {
            let (lower_free, upper_free) = match seek_free_memory_block_sized(memory_blocks, file_block.end_index - file_block.start_index) {
                Some((lower, upper)) => (lower, upper),
                None => {
                    //The memory block does not have a large enough free block to move the current file
                    continue;
                }
            };
            if lower_free > file_block.start_index {
                continue;
            }
            for (free_index, occupied_indx) in (lower_free..upper_free + 1).zip(file_block.start_index..file_block.end_index + 1) {
                memory_blocks.swap(free_index, occupied_indx);
            }
        }
    }

    fn compactify_memory_blocks_with_fragmentation(memory_blocks: &mut [MemoryBlock]) -> Option<()> {
        //! Swap individual memory blocks with fragmentation. 
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

fn seek_free_memory_block_sized(memory_blocks: &[MemoryBlock], required_size: usize) -> Option<(usize, usize)> {
    //! Return the start-> end index of a free memory block that is `required_size` large.
    //! If this is not possible, then `None` is returned.
    for (block_position, memory_block) in memory_blocks.iter().enumerate() {
        if let MemoryBlock::Occupied { .. } = memory_block {
            continue;
        }
        //Continue to increment the `upper_bound` until a not-free block is encountered or 
        //the required size is achieved.
        let lower_bound = block_position;
        let mut upper_bound = lower_bound;
        while let Some(MemoryBlock::Free) = memory_blocks.get(upper_bound) {
            if upper_bound - lower_bound == required_size {
                return Some((lower_bound, upper_bound));
            }
            upper_bound += 1;
        }
    }
    None
}

fn seek_next_free_memory_block(memory_blocks: &[MemoryBlock], lower_bound: usize) -> Option<usize> {
    //! Return the index of the next available free memory block if it exists, starting at the passed in `lower_bound`.
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
    //! Return the position of the next occupied memory block, starting at `upper_bound`.
    let mut seek_position = upper_bound as i64;
    while seek_position >= 0 {
        if let MemoryBlock::Occupied { .. } = memory_blocks.get(seek_position as usize).unwrap() {
            return Some(seek_position as usize)
        }
        seek_position -=1;
    }
    None
}

fn get_checksum(memory_blocks: &[MemoryBlock]) -> usize {
    let mut checksum = 0;
    for (position, memory_block) in memory_blocks.iter().enumerate() {
        if let MemoryBlock::Occupied { file_id } = memory_block {
            checksum += position * (*file_id);
        }
    } 
    checksum
}

impl SolveAdvent for Day9 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        //! Swap memory with file fragmentation allowed.
        let file_contents = read_input_file(path_to_file)?;
        let mut memory_blocks = MemoryBlock::construct_memory_blocks(&file_contents)?;
        MemoryBlock::compactify_memory_blocks_with_fragmentation(&mut memory_blocks);
        let checksum = get_checksum(&memory_blocks);
        println!("Final checksum after memory compaction is {}", checksum);
        Ok(())
    }

    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        //! Swap memory with file fragmentation not allowed, only whole files are moved
        let file_contents = read_input_file(path_to_file)?;
        let mut memory_blocks = MemoryBlock::construct_memory_blocks(&file_contents)?;
        MemoryBlock::compactify_memory_blocks_without_fragmentation(&mut memory_blocks);
        let checksum = get_checksum(&memory_blocks);
        println!("Final checksum after memory compaction is {}", checksum);
        Ok(())
    }
}
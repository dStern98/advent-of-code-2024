
use super::{read_input_file, SolveAdvent};

pub struct Day4;

///Wrapper around the word map so that methods
/// can be implemented.
struct WordMap(Vec<Vec<char>>);

impl WordMap {
    fn safe_read(&self, row: i64, col: i64) -> Option<char> {
        //! Facade to safely read from the `WordMap`. Takes row/col as i64
        //! so that we can go off the map without causing rust to panic (which a usize would do if it went negative in dev mode).
        if row < 0 || col < 0 {
            return None;
        }
        if let Some(row) = self.0.get(row as usize) {
            if let Some(item) = row.get(col as usize) {
                return Some(*item);
            }
        }
        None
    }

    fn read_diagonal_4_nw(&self, row: i64, col: i64) -> Option<String> {
        //! Get the string 4 Northwest (including the current base position) if it exists.
        let above: Option<Vec<_>> = [self.safe_read(row, col), self.safe_read(row -1, col - 1), self.safe_read(row - 2, col - 2), self.safe_read(row - 3, col - 3)].into_iter().collect();
        let above = above?;
        Some(above.into_iter().collect::<String>())
    }

    fn read_diagonal_4_ne(&self, row: i64, col: i64) -> Option<String> {
        //! Get the string 4 Northeast if it exists.
        let above: Option<Vec<_>> = [self.safe_read(row, col), self.safe_read(row - 1, col + 1), self.safe_read(row -2, col + 2), self.safe_read(row - 3, col + 3)].into_iter().collect();
        let above = above?;
        Some(above.into_iter().collect::<String>())
    }

    fn read_diagonal_4_sw(&self, row: i64, col: i64) -> Option<String> {
        //! Get the string 4 Southwest if it exists.
        let above: Option<Vec<_>> = [self.safe_read(row, col), self.safe_read(row + 1, col - 1), self.safe_read(row + 2, col - 2), self.safe_read(row + 3, col - 3)].into_iter().collect();
        let above = above?;
        Some(above.into_iter().collect::<String>())
    }

    fn read_diagonal_4_se(&self, row: i64, col: i64) -> Option<String> {
        //! Get the string 4 Southeast if it exists.
        let above: Option<Vec<_>> = [self.safe_read(row, col), self.safe_read(row + 1, col + 1), self.safe_read(row + 2, col + 2), self.safe_read(row + 3, col + 3)].into_iter().collect();
        let above = above?;
        Some(above.into_iter().collect::<String>())
    }

    fn read_4_left(&self, row: i64, col: i64) -> Option<String> {
        //! Get the string 4 left if it exists.
        let above: Option<Vec<_>> = [self.safe_read(row, col), self.safe_read(row, col - 1), self.safe_read(row, col - 2), self.safe_read(row, col - 3)].into_iter().collect();
        let above = above?;
        Some(above.into_iter().collect::<String>())
    }

    fn read_4_right(&self, row: i64, col: i64) -> Option<String> {
        //! Get the string 4 right if it exists.
        let above: Option<Vec<_>> = [self.safe_read(row, col), self.safe_read(row, col + 1), self.safe_read(row, col + 2), self.safe_read(row, col + 3)].into_iter().collect();
        let above = above?;
        Some(above.into_iter().collect::<String>())
    }

    fn read_4_below(&self, row: i64, col: i64) -> Option<String> {
        //! Get the string 4 below if it exists.
        let above: Option<Vec<_>> = [self.safe_read(row, col), self.safe_read(row + 1, col ), self.safe_read(row + 2, col), self.safe_read(row + 3, col)].into_iter().collect();
        let above = above?;
        Some(above.into_iter().collect::<String>())
    }


    fn read_4_above(&self, row: i64, col: i64) -> Option<String> {
        //! Get the string 4 above it it exists.
        let above: Option<Vec<_>> = [self.safe_read(row, col), self.safe_read(row - 1, col), self.safe_read(row - 2, col), self.safe_read(row - 3, col)].into_iter().collect();
        let above = above?;
        Some(above.into_iter().collect::<String>())
    }

}
impl SolveAdvent for Day4 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let word_search = WordMap(file_contents.lines().map(|line| line.trim().chars().collect::<Vec<_>>()).collect::<Vec<_>>());
        let mut xmas_word_count = 0;
        for row in 0..word_search.0.len() {
            for col in 0..word_search.0.first().unwrap().len() {
                let row = row as i64;
                let col = col as i64;
                //Read all 8 allowed cardinal directions from the base `row, col` position.
                let results_iterator = [
                    word_search.read_4_above(row, col), 
                    word_search.read_4_below(row, col),
                    word_search.read_4_left(row, col),
                    word_search.read_4_right(row, col), 
                    word_search.read_diagonal_4_ne(row, col), 
                    word_search.read_diagonal_4_nw(row, col), 
                    word_search.read_diagonal_4_se(row, col), 
                    word_search.read_diagonal_4_sw(row, col)
                ].into_iter().flatten();
                //Count all instances of 'XMAS'. Note that we do not count backwards instances,
                //as a future iteration of this loop will find this facing forward and we don't want to double count!
                for word_found in results_iterator {
                    if word_found == "XMAS" {
                        xmas_word_count += 1;
                    }
                }
            }
        }
        println!("XMAS word count: {:?}", xmas_word_count);
        Ok(())
    }

    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let word_search = WordMap(file_contents.lines().map(|line| line.trim().chars().collect::<Vec<_>>()).collect::<Vec<_>>());
        let mut xmases_count = 0;
        for row in 0..word_search.0.len() {
            for col in 0..word_search.0.first().unwrap().len() {
                if word_search.0[row][col] != 'A' {
                    continue;
                }
                //At this point we know that we are centered on an 'A',
                //we need to check the 4 diagonals to see if they form the required
                // X-MAS x shape.
                let row = row as i64;
                let col = col as i64;
                let diagonal_nw_to_se = [
                    word_search.safe_read(row - 1, col - 1), 
                    word_search.safe_read(row + 1, col + 1)
                ].into_iter().flatten().collect::<String>();
                let diagonal_ne_to_sw = [
                    word_search.safe_read(row - 1, col + 1),
                    word_search.safe_read(row + 1, col - 1)
                ].into_iter().flatten().collect::<String>();
                if (diagonal_ne_to_sw == "MS" || diagonal_ne_to_sw == "SM") && (diagonal_nw_to_se == "MS" || diagonal_nw_to_se == "SM") {
                        xmases_count += 1;
                    
                }
            }
        }
        println!("The total number of X-MAS's is {}", xmases_count);
        Ok(())
    }
}
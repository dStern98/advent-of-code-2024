use std::{collections::{HashMap, HashSet, VecDeque}, fmt::Display};

use super::{read_input_file, SolveAdvent};

pub struct Day21;

const NUMERIC_KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'], 
    ['4', '5', '6'], 
    ['1', '2', '3'], 
    [' ', '0', 'A']
    ];
const DIRECTIONAL_KEYPAD: [[char; 3]; 2] = [
    [' ', '^', 'A'], 
    ['<', 'V', '>']
    ];

///An Explorer collects possible paths to traverse
/// between two points in either the `DIRECTIONAL_KEYPAD`
/// or the `NUMERIC_KEYPAD`, ALWAYS IN TERMS of the DIRECTIONAL KEYPAD
#[derive(Debug, Clone)]
struct KeyPadExplorer {
    row: i64,
    col: i64,
    ///The move_history in terms of directional key inputs
    move_history: Vec<char>,
    visited: HashSet<(i64, i64)>
}

impl KeyPadExplorer {
    fn new_empty(row: i64, col: i64) -> Self {
        KeyPadExplorer {
            row,
            col,
            move_history: Vec::new(), 
            visited: HashSet::new()
        }
    }
    fn visit(&mut self) {
        self.visited.insert((self.row, self.col));
    }
    fn in_cycle(&self) -> bool {
        self.visited.contains(&(self.row, self.col))
    }
    fn new(row: i64, col: i64,  move_history: Vec<char>, visited: HashSet<(i64, i64)>) -> Self {
        KeyPadExplorer {
            row,
            col,
            move_history, 
            visited
        }
    }

    ///To enforce the rule that empty spaces can never be traversed, 
    /// returns `None` if either the current position if off the keypad
    /// or in the forbidden empty space position.
    fn safe_read(&self, keypad: &[[char; 3]]) -> Option<char> {
        if let Some(row) = keypad.get(self.row as usize) {
            if let Some(symbol) = row.get(self.col as usize) {
                if *symbol != ' ' {
                    return Some(*symbol);
                }
            }
        }
        None
    }

    fn spawn_next(&self) -> impl IntoIterator<Item=Self> {
        let mut next_items = [
            Self::new(self.row + 1, self.col,  self.move_history.clone(), self.visited.clone()),
            Self::new(self.row - 1, self.col,  self.move_history.clone(), self.visited.clone()),
            Self::new(self.row, self.col + 1,  self.move_history.clone(), self.visited.clone()),
            Self::new(self.row, self.col - 1, self.move_history.clone(), self.visited.clone())
        ];
        for (next_item, new_move) in  next_items.iter_mut().zip(['V', '^', '>', '<']) {
            next_item.move_history.push(new_move);
        }
        next_items
    }
}

///From the keypad position specified by `starting_row`, `starting_col`, collect all possible optimal paths between the start position
/// and all other positions in the given `keypad_in`.
fn translate_to_directional_inputs_from_start(starting_row: usize, starting_col: usize, keypad_in: &[[char; 3]]) -> HashMap<(char, char), Vec<String>> { 
    let mut decoded_moves: HashMap<(char, char), Vec<String>> = HashMap::new();
    let mut traversal_queue = VecDeque::new();
    let starting_explorer = KeyPadExplorer::new_empty(starting_row as i64, starting_col as i64);
    let starting_symbol = starting_explorer.safe_read(keypad_in).unwrap();
    traversal_queue.push_back(starting_explorer);
    while let Some(mut current_explorer) = traversal_queue.pop_front() {
        if current_explorer.in_cycle() {
            continue;
        }
        current_explorer.visit();
        if let Some(current_symbol) = current_explorer.safe_read(keypad_in) {
            decoded_moves.entry((starting_symbol, current_symbol)).or_default().push(current_explorer.move_history.clone().iter().collect::<String>());
            traversal_queue.extend(current_explorer.spawn_next());

        }
    }
    decoded_moves
}

///Find all possible optimal paths between any two valid positions
/// on the passed in `keypad_in`, always in terms of the directional keypad inputs.
fn translate_to_directional_inputs(keypad_in: &[[char; 3]]) -> HashMap<(char, char), Vec<String>> {
    let mut index = HashMap::new();
    for (row_number, row) in keypad_in.iter().enumerate() {
        for (col_number, symbol) in row.iter().enumerate() {
            if *symbol == ' ' {
                continue;
            }
           index.extend(translate_to_directional_inputs_from_start(row_number, col_number, keypad_in));
        }
    }

    for (_, move_history) in index.iter_mut() {
        //As an optimization, we only consider possible paths that are of the length of the shortest
        //possible path. 
        let shortest_path_length = move_history.iter().min_by_key(|path| path.len()).unwrap().len();
        move_history.retain(|path| path.len() == shortest_path_length);
        for move_option in move_history.iter_mut() {
            //Append the A to each command because we need to press the button once we have reached
            //the desired button
            move_option.push('A');
        }
    }
    index
}

///Represents a keypad input that must be translated into
/// a directional keypad command (1 layer deeper in the robot control chain)
#[derive(Debug)]
enum KeyPadInput {
    ///We are translating from a numerical keypad
    NumericalInput(String), 
    ///We are translating from a directional keypad
    DirectionalInput(String), 
}

impl Display for KeyPadInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_write = match self {
            Self::NumericalInput(to_decode) => to_decode,
            Self::DirectionalInput(to_decode) => to_decode
        };
        write!(f, "{}", to_write)
    }
}

impl KeyPadInput {
    ///Prepent the A symbol because all robots start pointing 
    /// at the `A` button for either numerical or directional keypad.
    fn prepend_start(&mut self) {
        match self {
            Self::NumericalInput(to_decode) => to_decode.insert(0, 'A'),
            Self::DirectionalInput(to_decode) => to_decode.insert(0, 'A')
        }
    }

    ///Expose the list of keypad inputs embedded in the `KeyPadInput`
    fn unbind(&self) -> &str {
        match self {
            Self::NumericalInput(to_decode) => to_decode,
            Self::DirectionalInput(to_decode) => to_decode
        }
    }
}

struct Decoder {
    directional_keypad_index: HashMap<(char, char), Vec<String>>,
    numerical_keypad_index: HashMap<(char, char), Vec<String>>
}

impl Decoder {
    fn new(directional_keypad_index: HashMap<(char, char), Vec<String>>, numerical_keypad_index: HashMap<(char, char), Vec<String>>) -> Self {
        Self {
            directional_keypad_index, 
            numerical_keypad_index
        }
    }

    ///Decode the passed in input, returning a list of all possible optimal directional
    /// keypad inputs. 
    fn decode(&self, to_decode: &KeyPadInput) -> Vec<KeyPadInput> {
        let (decoder, to_decode)= match to_decode {
            KeyPadInput::DirectionalInput(to_decode) => (&self.directional_keypad_index, to_decode), 
            KeyPadInput::NumericalInput(to_decode) => (&self.numerical_keypad_index, to_decode)
        };
        let mut collected_moves: Vec<String> = Vec::new();
        for (current, next) in to_decode.chars().zip(to_decode.chars().skip(1)) {
            let previous_moves = std::mem::take(&mut collected_moves);
            if let Some(next_moves) = decoder.get(&(current, next)) {
                for next_move in next_moves {
                    if !previous_moves.is_empty() {
                        for previous_move in previous_moves.iter() {
                            collected_moves.push(format!("{}{}", previous_move, &next_move));
                        }
                    } else {
                        collected_moves.extend(next_moves.clone());
                    }
                }
            } else {
                panic!("Decoder did not contain a mapping from {current} => {next}")
            }
        }
        collected_moves.into_iter().map(KeyPadInput::DirectionalInput).collect::<Vec<_>>()
    }
}

fn numerical_code_component(code_to_input: &str) -> i64{
    //! Extract the numerical component of the code to input
    let mut valid_numbers = code_to_input.chars().filter_map(|char| {
        char.to_digit(10)
    }).collect::<Vec<_>>();
    let mut non_zero_number_seen = false;
    valid_numbers.retain(|num| {
        if non_zero_number_seen {
            return true;
        } else if *num != 0 {
            non_zero_number_seen = true;
            return true;
        }
        false
    });
    valid_numbers.into_iter().map(|num| num.to_string()).collect::<String>().parse::<i64>().unwrap_or(0)
}

fn decode_password(decoder: &Decoder, code_to_input: &str, decode_layers: usize) -> String {
    //! Decode the passed in `code_to_input` (which is a numerical keypad task) into an optimal
    //! set of directional keypad inputs. The `decode_layers` variable specifies how many layers of directional keypads
    //! are in the way of the numerical keypad
    let mut codes_to_input = vec![KeyPadInput::NumericalInput(code_to_input.to_string())];
    for _ in 0..decode_layers {
        let mut shortest_decoded_code = usize::MAX;
        let codes_to_decode = std::mem::take(&mut codes_to_input);
        for mut code_to_decode in codes_to_decode {
            code_to_decode.prepend_start();
            let directional_keypad_options = decoder.decode(&code_to_decode);
            for possible_path in directional_keypad_options.iter() {
                shortest_decoded_code = shortest_decoded_code.min(possible_path.unbind().len());
            }
            codes_to_input.extend(directional_keypad_options);
        }
        //As a performance optimization, remove any possible directional keypad inputs that are not 
        //tied for the least number of inputs
        codes_to_input.retain(|possible_key_pad_input| possible_key_pad_input.unbind().len() <= shortest_decoded_code);
    }
    codes_to_input.iter().min_by_key(|key_pad_input| key_pad_input.unbind().len()).unwrap().unbind().to_string()

} 

impl SolveAdvent for Day21 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let indexed_direction_keypad = translate_to_directional_inputs(&DIRECTIONAL_KEYPAD);
        let indexed_numerical_keypad = translate_to_directional_inputs(&NUMERIC_KEYPAD);
        let decoder = Decoder::new(indexed_direction_keypad, indexed_numerical_keypad);
        let mut total_complexity = 0;
        for line in file_contents.lines() {
            println!("----------------------------------------");
            let numerical_component = numerical_code_component(line);
            let decoded_result = decode_password(&decoder, line, 3);
            println!("Optimal result length {} with numerical component {} for line {}", decoded_result.len(), numerical_component, line);
            total_complexity += numerical_component as usize * decoded_result.len(); 
        }
        println!("Sum of all complexities: {}", total_complexity);
        Ok(())
    }

    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
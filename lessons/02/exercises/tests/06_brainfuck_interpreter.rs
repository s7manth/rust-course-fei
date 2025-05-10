//! Run this file with `cargo test --test 06_brainfuck_interpreter`.

// DONE (bonus): Create an interpreter for the [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) language.
// The Brainfuck program will be parsed out of a string and represented as a struct.
//
// Handle both parsing and execution errors using enums representing error conditions,
// see tests for details.
// A parsing error can be either an unknown instruction or an unpaired loop instruction.
// An execution error can be either that the program tries to read input, but there is no more
// input available, or when the program executes more than 10000 instructions (which probably
// signals an infinite loop).
//
// Hint: Put `#[derive(Debug, Eq, PartialEq)]` on top of `ParseError`, `ExecuteError` and `Program`
// (and any other custom types nested inside them) so that asserts in tests work.

use std::collections::{HashSet, HashMap};

#[derive(Debug, Eq, PartialEq)]
enum ParseError {
    UnknownInstruction {
        location: u32,
        instruction: char
    },
    UnmatchedLoop {
        location: u32
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ExecuteError {
    NoInputLeft, 
    InfiniteLoop
}

// #[derive(Debug, Eq, PartialEq)]
// enum BrainfuckError {
//     Pa(ParseError),
//     Ex(ExecuteError),
// }

// impl From<ParseError> for BrainfuckError {
//     fn from(e: ParseError) -> Self {
//         GenericError::Pa(e)
//     }
// }

// impl From<ExecuteError> for BrainfuckError {
//     fn from(e: ExecuteError) -> Self {
//         GenericError::Ex(e)
//     }
// }

#[derive(Debug, Eq, PartialEq)]
struct Program<'a> {
    program_text: &'a str,
}

#[derive(Debug, Eq, PartialEq)]
struct ProgramState {
    program_index: usize,
    argument_index: usize,
    loop_index_st: Vec<usize>,
    loop_index_en: usize,
    in_loop: bool,
    focus: usize,
    values: Vec<u8>,
    output: String,
    num_instructions: u32,
}

impl<'a> Program<'a> {
    fn execute(&self, args: Vec<u8>, _nums: Vec<i32>) -> Result<String, ExecuteError> {
        if self.program_text.is_empty() {
            return Ok(self.program_text.to_string());
        }

        // process loops to determine if infinite loop
        // if [, ] exists in the program text 
        // extract whats between them 

        let mut state = ProgramState {
            program_index: 0,
            argument_index: 0,
            loop_index_st: vec![],
            loop_index_en: 0,
            in_loop: false,
            focus: 0,
            values: vec![0],
            output: "".to_string(),
            num_instructions: 0,
        };

        let program: Vec<_> = self.program_text.chars().collect();

        while state.program_index < program.len() {
            if state.num_instructions == 10000 {
                return Err(ExecuteError::InfiniteLoop);
            }

            println!("{:?}", state);
            match program[state.program_index] {
                '>' => {
                    if state.focus == state.values.len() - 1 {
                        state.values.push(0);
                    }
                    state.focus += 1;
                }
                '<' => { 
                    state.focus = state.focus.saturating_sub(1);
                }
                '[' => {
                    state.loop_index_st.push(state.program_index);

                    if state.values[state.focus] == 0 {
                        state.in_loop = false;
                        state.loop_index_st.pop();
                        state.program_index = state.loop_index_en;
                    }
                },
                ']' => {
                    state.loop_index_en = state.program_index;

                    if state.values[state.focus] != 0 {
                        state.program_index = state.loop_index_st[state.loop_index_st.len() - 1];
                    } else {
                        state.loop_index_st.pop();
                    }
                },
                '+' => {
                    state.values[state.focus] = state.values[state.focus].wrapping_add(1);
                },
                '.' => {
                    state.output = format!("{}{}", state.output, char::from(state.values[state.focus]));
                },
                ',' => {
                    if state.argument_index >= args.len() {
                        return Err(ExecuteError::NoInputLeft);
                    }

                    state.values[state.focus] = args[state.argument_index];
                    state.argument_index += 1;
                },
                '-' => {
                    state.values[state.focus] = state.values[state.focus].wrapping_sub(1);
                },
                _ => {}
            }

            state.program_index += 1;
            state.num_instructions += 1;
        }

        Ok(state.output)
    }
}

fn parse_program(program_text: &str) -> Result<Program, ParseError> {
    if program_text.is_empty() {
        return Ok(Program {
            program_text
        });
    }

    // check for invalid characters in the program
    let legal_characters = HashSet::from(['>', '<', '[', ']', '+', '.', ',', '-']);
    match program_text.find(|c: char| !legal_characters.contains(&c)) {
        Some(index) => return Err(ParseError::UnknownInstruction {
            location: index as u32,
            instruction: program_text.chars().nth(index).unwrap()
        }),
        None => {}
    }

    // check for closing square bracket pairs
    let dict = HashMap::from([
        ('[', ']'),
        (']', 'x'),
    ]);

    let mut stack: Vec<char> = Vec::new();
    let mut prev = 0;

    for t in program_text.chars().enumerate() {
        let b = t.1;
        if !dict.contains_key(&b) { continue; }
        prev = t.0;

        match dict.get(&b) == Some(&']') {
            true => stack.push(b),
            false => {
                if stack.len() == 0 {
                    return Err(ParseError::UnmatchedLoop {
                        location: prev as u32,
                    });
                }

                // validation to check
                let last = stack.get(stack.len() - 1).unwrap();
                if Some(&b) == dict.get(last) {
                    stack.pop();
                } else {
                    return Err(ParseError::UnmatchedLoop {
                        location: prev as u32,
                    });
                }
            }
        }
    }

    match stack.len() == 0 {
        true => Ok(Program {
            program_text
        }),
        false => Err(ParseError::UnmatchedLoop {
            location: prev as u32,
        })
    }
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::{parse_program, ExecuteError, ParseError};

    #[test]
    fn parse_empty() {
        check_output("", "", "");
    }

    #[test]
    fn parse_unknown_instruction() {
        assert!(matches!(
            parse_program(">p"),
            Err(ParseError::UnknownInstruction {
                location: 1,
                instruction: 'p'
            })
        ));
    }

    #[test]
    fn parse_unmatched_loop_start() {
        assert_eq!(
            parse_program(">++[+>][++>"),
            Err(ParseError::UnmatchedLoop { location: 7 })
        );
    }

    #[test]
    fn parse_unmatched_loop_end() {
        assert_eq!(
            parse_program(">++[+>][++>]+]"),
            Err(ParseError::UnmatchedLoop { location: 13 })
        );
    }

    #[test]
    fn missing_input() {
        let program = parse_program(",").unwrap();
        let result = program.execute(vec![], vec![0; 30000]);
        assert_eq!(result, Err(ExecuteError::NoInputLeft));
    }

    #[test]
    fn infinite_loop() {
        let program = parse_program("+[]").unwrap();
        let result = program.execute(vec![], vec![0; 30000]);
        assert_eq!(result, Err(ExecuteError::InfiniteLoop));
    }

    #[test]
    fn copy_input() {
        check_output(",.>,.>,.>,.>,.", "hello", "hello");
    }

    #[test]
    fn output_exclamation_mark() {
        check_output("+++++++++++++++++++++++++++++++++.", "", "!");
    }

    #[test]
    fn three_exclamation_marks() {
        check_output(">+++++++++++++++++++++++++++++++++<+++[>.<-]", "", "!!!");
    }

    #[test]
    fn hello_world() {
        check_output("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.", "", "Hello World!\n");
    }

    fn check_output(program_text: &str, input: &str, expected_output: &str) {
        let program = parse_program(program_text);
        match program {
            Ok(program) => {
                let result = program
                    .execute(input.to_string().into_bytes(), vec![0; 30000])
                    .expect(&format!("Cannot execute program {program_text}"));
                assert_eq!(result, expected_output);
            }
            Err(error) => {
                panic!("Error occurred while parsing program {program_text}: {error:?}");
            }
        }
    }
}

use std::env;
use std::io::{self, Write};

const MEMORY_SIZE: usize = 2048;

fn brain_fuck(code: &str) {
    let mut memory = vec![0u8; MEMORY_SIZE];
    let mut data_pointer = 0;
    let code_chars: Vec<char> = code.chars().collect();
    let mut code_pointer = 0;
    let code_length = code_chars.len();

    while code_pointer < code_length {
        match code_chars[code_pointer] {
            '>' => {
                data_pointer = (data_pointer + 1) % MEMORY_SIZE;
            }
            '<' => {
                if data_pointer == 0 {
                    data_pointer = MEMORY_SIZE - 1;
                } else {
                    data_pointer -= 1;
                }
            }
            '+' => {
                memory[data_pointer] = memory[data_pointer].wrapping_add(1);
            }
            '-' => {
                memory[data_pointer] = memory[data_pointer].wrapping_sub(1);
            }
            '.' => {
                print!("{}", memory[data_pointer] as char);
                io::stdout().flush().unwrap();
            }
            '[' => {
                if memory[data_pointer] == 0 {
                    let mut open_brackets = 1;
                    while open_brackets > 0 {
                        code_pointer += 1;
                        if code_pointer >= code_length {
                            break;
                        }
                        if code_chars[code_pointer] == '[' {
                            open_brackets += 1;
                        } else if code_chars[code_pointer] == ']' {
                            open_brackets -= 1;
                        }
                    }
                }
            }
            ']' => {
                if memory[data_pointer] != 0 {
                    let mut close_brackets = 1;
                    while close_brackets > 0 {
                        if code_pointer == 0 {
                            break;
                        }
                        code_pointer -= 1;
                        if code_chars[code_pointer] == '[' {
                            close_brackets -= 1;
                        } else if code_chars[code_pointer] == ']' {
                            close_brackets += 1;
                        }
                    }
                }
            }
            _ => {} // Ignore comments and other characters
        }
        code_pointer += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        brain_fuck(&args[1]);
    } else {
        eprintln!("Usage: {} <Brainfuck code>", args[0]);
    }
}
